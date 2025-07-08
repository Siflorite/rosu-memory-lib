pub mod info;
pub mod offset;
pub mod location;

use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::beatmap::common::BeatmapInfo;
use crate::reader::structs::State;
use crate::reader::beatmap::stable::offset::*;
use std::collections::HashSet;

use crate::reader::beatmap::common::BeatmapLocation;
use crate::reader::beatmap::common::BeatmapStats;
use crate::reader::beatmap::common::BeatmapTechnicalInfo;
use crate::common::GameMode;
use crate::reader::beatmap::common::BeatmapStatus;
use crate::reader::beatmap::common::BeatmapMetadata;

use log::{info, warn};
use simplelog::*;
use std::fs::File;

pub(crate) fn get_beatmap_addr(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    let beatmap_ptr = p.read_i32(state.addresses.base - BEATMAP_OFFSET.ptr)?;
    Ok(p.read_i32(beatmap_ptr)?)
}

pub(crate) fn read_from_beatmap_ptr_string(p: &Process, state: &mut State, offset: i32) -> eyre::Result<String>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;
    Ok(p.read_string(beatmap_addr + offset)?)
}


pub fn get_beatmap_info(p: &Process, state: &mut State) -> eyre::Result<BeatmapInfo>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;

    // done like that to be more efficient reading the string one by one would need to reload addr everytime which cost more
    Ok(BeatmapInfo {
        technical: BeatmapTechnicalInfo{
            md5: p.read_string(beatmap_addr + BEATMAP_OFFSET.technical.md5)?,
            id: p.read_i32(beatmap_addr + BEATMAP_OFFSET.technical.id)?,
            set_id: p.read_i32(beatmap_addr + BEATMAP_OFFSET.technical.set_id)?,
            mode: GameMode::Osu,
            ranked_status: BeatmapStatus::from(p.read_i32(beatmap_addr + BEATMAP_OFFSET.technical.ranked_status)?),
        },
        metadata: BeatmapMetadata{
            author: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.author)?,
            creator: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.creator)?,
            title_romanized: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.title_romanized)?,
            title_original: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.title_original)?,
            difficulty: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.difficulty)?,
            tags: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.tags)?,
        },
        stats: BeatmapStats{
            ar: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.ar)?,
            od: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.od)?,
            cs: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.cs)?,
            hp: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.hp)?,
            total_length: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.total_length)?,
            star_rating: 0.0,
            object_count: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.object_count)?,
        },
        location: BeatmapLocation {
            folder: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.folder)?,
            filename: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.filename)?,
            audio: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.audio)?,
            cover: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.cover)?,
        },
    })
}

const TARGET_VALUE: i32 = 86;
const TARGET_FLOAT_MIN: f32 = 5.10;
const TARGET_FLOAT_MAX: f32 = 5.12;

#[derive(Debug)]
struct FoundValue {
    offset: i32,
    value_type: String,
    value: String,
    step_size: i32,
}

fn get_known_offsets() -> HashSet<i32> {
    let mut offsets = HashSet::new();
    
    // Location offsets
    offsets.insert(BEATMAP_LOCATION_OFFSET.folder);
    offsets.insert(BEATMAP_LOCATION_OFFSET.filename);
    offsets.insert(BEATMAP_LOCATION_OFFSET.audio);
    offsets.insert(BEATMAP_LOCATION_OFFSET.cover);

    // Stats offsets (skip star_rating car TODO)
    offsets.insert(BEATMAP_STATS_OFFSET.ar);
    offsets.insert(BEATMAP_STATS_OFFSET.od);
    offsets.insert(BEATMAP_STATS_OFFSET.cs);
    offsets.insert(BEATMAP_STATS_OFFSET.hp);
    offsets.insert(BEATMAP_STATS_OFFSET.object_count);
    offsets.insert(BEATMAP_STATS_OFFSET.total_length);

    // Technical offsets (skip mode car TODO)
    offsets.insert(BEATMAP_TECHNICAL_OFFSET.md5);
    offsets.insert(BEATMAP_TECHNICAL_OFFSET.id);
    offsets.insert(BEATMAP_TECHNICAL_OFFSET.set_id);
    offsets.insert(BEATMAP_TECHNICAL_OFFSET.ranked_status);

    // Metadata offsets
    offsets.insert(BEATMAP_METADATA_OFFSET.author);
    offsets.insert(BEATMAP_METADATA_OFFSET.creator);
    offsets.insert(BEATMAP_METADATA_OFFSET.title_romanized);
    offsets.insert(BEATMAP_METADATA_OFFSET.title_original);
    offsets.insert(BEATMAP_METADATA_OFFSET.difficulty);
    offsets.insert(BEATMAP_METADATA_OFFSET.tags);

    // Other known offsets
    offsets.insert(BEATMAP_OFFSET.ptr);

    offsets
}

pub fn test_get_memory_dump(p: &Process, state: &mut State) -> eyre::Result<()>
{
    // Setup logging
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("memory_scan.log")?
        ),
    ])?;

    let beatmap_addr = get_beatmap_addr(p, state)?;
    let base_offset = 0x0;
    let mut found_values = Vec::new();
    let known_offsets = get_known_offsets();

    info!("Starting memory scan at base address: {:#x}", beatmap_addr);
    info!("Base offset: {:#x}", base_offset);
    info!("Scanning with step size of 2 bytes");
    info!("Excluding {} known offsets", known_offsets.len());

    // Scan 200 offsets after base_offset
    for i in 0..200 {
        let current_offset = base_offset + (i * 2);
        
        // Skip if this is a known offset
        if known_offsets.contains(&current_offset) {
            info!("Skipping known offset {:#x}", current_offset);
            continue;
        }

        let addr = beatmap_addr + current_offset;
        
        // Try reading as f32 first since we're looking for a float
        if let Ok(val) = p.read_f32(addr) {
            if val >= TARGET_FLOAT_MIN && val <= TARGET_FLOAT_MAX {
                info!("\n!!! FOUND float {:.3} at Offset {:#x} (Address: {:#x}) !!!", val, current_offset, addr);
                found_values.push(FoundValue {
                    offset: current_offset,
                    value_type: "float".to_string(),
                    value: format!("{:.3}", val),
                    step_size: 2,
                });
            } else if val >= -1000.0 && val <= 1000.0 {
                info!("\nOffset {:#x} (Address: {:#x}):", current_offset, addr);
                info!("  f32: {:.3}", val);
            }
        }

        // Try reading as i32 and then as float if it's a pointer
        if let Ok(val) = p.read_i32(addr) {
            if val == TARGET_VALUE {
                info!("  i32: {} (TARGET VALUE FOUND!)", val);
                found_values.push(FoundValue {
                    offset: current_offset,
                    value_type: "i32".to_string(),
                    value: val.to_string(),
                    step_size: 2,
                });
            } else {
                info!("  i32: {}", val);
            }
            
            // Si on trouve un pointeur valide, essayons de lire à cette adresse aussi
            if val > 0x10000 { // Probablement un pointeur valide
                if let Ok(pointed_float) = p.read_f32(val) {
                    if pointed_float >= TARGET_FLOAT_MIN && pointed_float <= TARGET_FLOAT_MAX {
                        info!("  !!! FOUND float {:.3} in pointed value !!!", pointed_float);
                        found_values.push(FoundValue {
                            offset: current_offset,
                            value_type: "pointed_float".to_string(),
                            value: format!("{:.3} (pointed from {:#x})", pointed_float, val),
                            step_size: 2,
                        });
                    }
                }
            }
        }

        // Try reading as string
        if let Ok(val) = p.read_string(addr) {
            if !val.is_empty() && val.chars().all(|c| c.is_ascii()) {
                info!("  string: {}", val);
            }
        }
    }

    // Print summary of found values
    if !found_values.is_empty() {
        info!("\n=== SUMMARY OF FOUND VALUES ===");
        info!("Found {} interesting values:", found_values.len());
        for found in found_values {
            info!("Offset: {:#x} (step: {})", found.offset, found.step_size);
            info!("  Type: {}", found.value_type);
            info!("  Value: {}", found.value);
            info!("  Relative to base: +{:#x}", found.offset - base_offset);
            info!("");
        }
    } else {
        warn!("\nNo target values found in scan.");
    }

    info!("Scan complete. Results saved to memory_scan.log");
    Ok(())
}