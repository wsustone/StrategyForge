# StrategyForge Blender Sprite Generator

This directory contains scripts for generating isometric sprites for the StrategyForge game using Blender.

## How to Use the Blender Isometric Sprite Generator

### Setup

1. Open Blender (version 2.8 or higher recommended)
2. Go to `Edit > Preferences > Add-ons` and make sure "Import-Export: Python Script Import/Export" is enabled
3. In Blender, open a new or existing project

### Importing the Script

1. In Blender, go to `Scripting` tab
2. Click `Open` and navigate to `blender_isometric_exporter.py`
3. Alternatively, you can copy-paste the script content into the Blender script editor

### Running the Script

1. With the script open in the script editor, click the `Run Script` button
2. The script will present a menu in the Blender console with options:
   - Create and render a test tank
   - Create and render a test aircraft  
   - Create and render a test resource
   - Render an existing object
   - Exit

### Creating Custom Models

For custom models:

1. Create your 3D model in Blender
2. Make sure your model:
   - Is centered at the origin
   - Has the correct scale (around 1-3 Blender units for most units)
   - Has applied transformations (`Ctrl+A > All Transformations`)
3. Run the script and choose option 4 to render an existing object
4. Enter the name of your model when prompted

### Advanced Usage

You can also use the `IsometricSpriteGenerator` class directly in your own scripts:

```python
# Import the class
from blender_isometric_exporter import IsometricSpriteGenerator

# Create a generator instance
generator = IsometricSpriteGenerator(output_path="C:/path/to/output")

# Set up the scene
generator.setup_scene()

# Render a sprite from all 4 directions
generator.render_sprite_rotations("YourModelName")

# Or render from specific angle
generator.render_sprite("YourModelName", filename="custom_name.png", rotation=45)
```

### Output

Sprites will be saved to a folder called "exported_sprites" in the same directory as your Blender file, unless you specify a different output path.

## Next Steps

1. Create your game units as Blender models
2. Generate sprites for each unit at the desired angles
3. Import the sprites into your game engine
