

pub fn get_sessions_plays(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    println!("Base: {:#x}", state.addresses.base);
    let beatmap_ptr = p.read_i32(state.addresses.base - 0x33)?;
    println!("Beatmap ptr: {:#x}", beatmap_ptr);
    Ok(p.read_i32(beatmap_ptr + 0xC)?)
}