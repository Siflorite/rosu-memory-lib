import rosu_memory_python

# Print the module documentation
print("Module documentation:")
print(rosu_memory_python.__doc__)
print()

try:
    # Try to initialize with a 100ms interval
    print("Initializing memory reader with 100ms interval...")
    state, process = rosu_memory_python.init_loop(100)
    print("Successfully initialized!")
    
    # Get beatmap info
    beatmap_info = rosu_memory_python.get_beatmap_info(process, state)
    
    # Print metadata
    print("\nBeatmap Metadata:")
    print(f"Author: {beatmap_info.metadata.author}")
    print(f"Creator: {beatmap_info.metadata.creator}")
    print(f"Title (Romanized): {beatmap_info.metadata.title_romanized}")
    print(f"Title (Original): {beatmap_info.metadata.title_original}")
    print(f"Difficulty: {beatmap_info.metadata.difficulty}")
    
    # Print stats
    print("\nBeatmap Stats:")
    print(f"AR: {beatmap_info.stats.ar}")
    print(f"OD: {beatmap_info.stats.od}")
    print(f"CS: {beatmap_info.stats.cs}")
    print(f"HP: {beatmap_info.stats.hp}")
    print(f"Length: {beatmap_info.stats.total_length}")
    print(f"Objects: {beatmap_info.stats.object_count}")
    print(f"Sliders: {beatmap_info.stats.slider_count}")
    
    # Print star ratings
    print("\nStar Ratings:")
    print(f"NoMod: {beatmap_info.stats.star_rating.no_mod}")
    print(f"DT: {beatmap_info.stats.star_rating.dt}")
    print(f"HT: {beatmap_info.stats.star_rating.ht}")
    
    # Print technical info
    print("\nTechnical Info:")
    print(f"MD5: {beatmap_info.technical.md5}")
    print(f"Beatmap ID: {beatmap_info.technical.id}")
    print(f"Beatmap Set ID: {beatmap_info.technical.set_id}")
    print(f"Mode: {beatmap_info.technical.mode}")
    print(f"Ranked Status: {beatmap_info.technical.ranked_status}")
    
    # Print location info
    print("\nLocation Info:")
    print(f"Folder: {beatmap_info.location.folder}")
    print(f"Filename: {beatmap_info.location.filename}")
    print(f"Audio: {beatmap_info.location.audio}")
    print(f"Cover: {beatmap_info.location.cover}")
    
except Exception as e:
    print(f"Error occurred: {e}") 