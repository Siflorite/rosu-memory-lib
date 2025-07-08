pub mod info;
pub mod offset;
pub mod location;

use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::beatmap::common::BeatmapInfo;
use crate::reader::structs::State;
use crate::reader::beatmap::stable::offset::BEATMAP_OFFSET;
use crate::reader::beatmap::common::BeatmapLocation;
use crate::reader::beatmap::common::BeatmapStats;
use crate::reader::beatmap::common::BeatmapTechnicalInfo;
use crate::common::GameMode;
use crate::reader::beatmap::common::BeatmapStatus;
use crate::reader::beatmap::common::BeatmapMetadata;

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



pub fn test_get_memory_dump(p: &Process, state: &mut State) -> eyre::Result<()>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;
    let base_offset = 0xFC; // latest Offset for Objectcount based on Tosu and gosu

    // Test different step sizes
    for step in [4, 6, 8].iter() {
        println!("\n=== Scanning with step size {} ===", step);
        
        // Scan 50 offsets after base_offset
        for i in 0..50 {
            let current_offset = base_offset + (i * step);
            let addr = beatmap_addr + current_offset;
            
            // Try reading as i32
            if let Ok(val) = p.read_i32(addr) {
                if val == 86 {
                    println!("\n!!! FOUND 86 at Offset {:#x} (Address: {:#x}) !!!", current_offset, addr);
                } else {
                    println!("\nOffset {:#x} (Address: {:#x}):", current_offset, addr);
                }
                println!("  i32: {}", val);
                
                // Si on trouve un pointeur valide, essayons de lire à cette adresse aussi
                if val > 0x10000 { // Probablement un pointeur valide
                    if let Ok(pointed_val) = p.read_i32(val) {
                        println!("  pointed i32: {}", pointed_val);
                        if pointed_val == 86 {
                            println!("  !!! FOUND 86 in pointed value !!!");
                        }
                    }
                }
            }

            // Try reading as f32
            if let Ok(val) = p.read_f32(addr) {
                if val >= -1000.0 && val <= 1000.0 { // Filter out nonsensical float values
                    println!("  f32: {}", val);
                }
            }

            // Try reading as string
            if let Ok(val) = p.read_string(addr) {
                if !val.is_empty() && val.chars().all(|c| c.is_ascii()) {
                    println!("  string: {}", val);
                }
            }

            // Try reading as pointer then string
            if let Ok(ptr) = p.read_i32(addr) {
                if ptr > 0x10000 { // Probablement un pointeur valide
                    if let Ok(val) = p.read_string(ptr) {
                        if !val.is_empty() && val.chars().all(|c| c.is_ascii()) {
                            println!("  pointer->string: {}", val);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}