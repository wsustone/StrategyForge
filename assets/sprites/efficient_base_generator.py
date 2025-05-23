import bpy
import math
import os
from mathutils import Vector

# Clear the scene
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete()

# Path configuration
output_dir = r"C:\Users\Juston\Documents\blender"
if not os.path.exists(output_dir):
    try:
        os.makedirs(output_dir)
    except PermissionError:
        print(f"Permission error creating directory: {output_dir}")
        import tempfile
        output_dir = os.path.join(tempfile.gettempdir(), "strategy_forge_sprites")
        if not os.path.exists(output_dir):
            os.makedirs(output_dir)
        print(f"Using temporary directory for sprites: {output_dir}")

def create_simple_building(building_type, size=1.0):
    """Create a simple building using basic shapes"""
    bpy.ops.object.select_all(action='DESELECT')
    
    if building_type == 'command_center':
        # Main building - larger with antenna
        bpy.ops.mesh.primitive_cube_add(size=size)
        main = bpy.context.active_object
        main.scale = (2.0, 2.0, 1.0)
        
        # Add antenna
        bpy.ops.mesh.primitive_cylinder_add(vertices=6, radius=0.1, depth=1.0)
        antenna = bpy.context.active_object
        antenna.location = (0, 0, 1.5)
        
        # Add dome
        bpy.ops.mesh.primitive_uv_sphere_add(radius=0.8, location=(0, 0, 1.0))
        dome = bpy.context.active_object
        dome.scale = (1, 1, 0.6)
        
        color = (0.8, 0.2, 0.2, 1.0)  # Red
        
    elif building_type == 'barracks':
        # Barracks - long rectangular building
        bpy.ops.mesh.primitive_cube_add(size=size)
        main = bpy.context.active_object
        main.scale = (2.5, 1.5, 0.8)
        
        # Add roof
        bpy.ops.mesh.primitive_cone_add(vertices=4, radius1=1.5, radius2=1.5, depth=2.5)
        roof = bpy.context.active_object
        roof.location.z = 0.8
        roof.rotation_euler = (0, 0, math.radians(45))
        
        color = (0.2, 0.2, 0.8, 1.0)  # Blue
        
    elif building_type == 'factory':
        # Factory - large industrial building
        bpy.ops.mesh.primitive_cube_add(size=size)
        main = bpy.context.active_object
        main.scale = (2.0, 3.0, 1.2)
        
        # Add smokestack
        bpy.ops.mesh.primitive_cylinder_add(vertices=8, radius=0.3, depth=1.5)
        stack = bpy.context.active_object
        stack.location = (1.0, 0, 1.6)
        
        # Add roof
        bpy.ops.mesh.primitive_cube_add(size=1.0)
        roof = bpy.context.active_object
        roof.scale = (2.2, 3.2, 0.1)
        roof.location.z = 1.7
        
        color = (0.4, 0.4, 0.4, 1.0)  # Gray
        
    elif building_type == 'power_plant':
        # Power plant - circular with cooling towers
        bpy.ops.mesh.primitive_cylinder_add(vertices=12, radius=1.5, depth=1.0)
        main = bpy.context.active_object
        
        # Add cooling towers
        for x in [-1, 1]:
            bpy.ops.mesh.primitive_cylinder_add(vertices=8, radius=0.5, depth=1.5)
            tower = bpy.context.active_object
            tower.location = (x * 0.8, 0, 1.0)
        
        color = (0.8, 0.8, 0.2, 1.0)  # Yellow
    
    # Set material for all objects
    for obj in bpy.context.selected_objects:
        mat = bpy.data.materials.new(name=f"{building_type}_material")
        mat.diffuse_color = color
        if obj.data.materials:
            obj.data.materials[0] = mat
        else:
            obj.data.materials.append(mat)
    
    # Select all parts and join them
    bpy.ops.object.select_all(action='DESELECT')
    for obj in bpy.data.objects:
        if obj.type == 'MESH':
            obj.select_set(True)
    bpy.context.view_layer.objects.active = bpy.context.selected_objects[0]
    bpy.ops.object.join()
    
    # Name the building
    bpy.context.active_object.name = f"{building_type.replace('_', ' ').title()}"
    
    return bpy.context.active_object

def setup_isometric_camera():
    """Set up an isometric camera"""
    # Remove any existing cameras
    bpy.ops.object.select_by_type(type='CAMERA')
    bpy.ops.object.delete()
    
    # Create new camera
    bpy.ops.object.camera_add()
    camera = bpy.context.active_object
    camera.name = 'IsometricCamera'
    
    # Set isometric angle
    camera.rotation_euler = (math.radians(54.736), 0, math.radians(45))
    
    # Position the camera
    camera.location = (12, -12, 12)  # Slightly higher for buildings
    
    # Set as active camera
    bpy.context.scene.camera = camera
    
    return camera

def setup_simple_lighting():
    """Create a simple lighting setup"""
    # Clear existing lights
    bpy.ops.object.select_by_type(type='LIGHT')
    bpy.ops.object.delete()
    
    # Add key light (sun)
    bpy.ops.object.light_add(type='SUN', radius=1, location=(0, 0, 10))
    key_light = bpy.context.active_object
    key_light.rotation_euler = (math.radians(45), 0, math.radians(45))
    key_light.data.energy = 2.0
    
    # Add fill light (hemi)
    bpy.ops.object.light_add(type='SUN', radius=1, location=(0, 0, 10))
    fill_light = bpy.context.active_object
    fill_light.rotation_euler = (math.radians(60), 0, math.radians(-30))
    fill_light.data.energy = 0.8

def setup_fast_render():
    """Configure render settings for speed"""
    # Set render engine to Eevee for faster rendering
    bpy.context.scene.render.engine = 'BLENDER_EEVEE_NEXT'
    
    # Resolution
    bpy.context.scene.render.resolution_x = 256
    bpy.context.scene.render.resolution_y = 256
    bpy.context.scene.render.resolution_percentage = 100
    
    # Performance settings
    bpy.context.scene.render.use_persistent_data = True
    
    # Simplify scene (for faster rendering)
    bpy.context.scene.render.use_simplify = True
    bpy.context.scene.render.simplify_subdivision = 0
    bpy.context.scene.render.simplify_child_particles = 0
    
    # Transparent background
    bpy.context.scene.render.film_transparent = True
    
    # Anti-aliasing
    bpy.context.scene.eevee.taa_render_samples = 16
    bpy.context.scene.eevee.taa_samples = 8

def render_8_angles(obj, output_dir):
    """Render an object from 8 angles (45° increments)"""
    # Make sure output directory exists
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    
    # Set output format
    bpy.context.scene.render.image_settings.file_format = 'PNG'
    bpy.context.scene.render.image_settings.color_mode = 'RGBA'
    
    # Get object name for filename
    obj_name = obj.name.split('.')[0]
    
    # Render from 8 angles (45° increments)
    for i in range(8):
        angle = i * (2 * math.pi / 8)
        obj.rotation_euler = (0, 0, angle)
        
        # Update scene
        bpy.context.view_layer.update()
        
        # Set output path
        output_path = os.path.join(output_dir, f"{obj_name}_angle_{i:02d}.png")
        bpy.context.scene.render.filepath = output_path
        
        # Render
        bpy.ops.render.render(write_still=True)
        print(f"Rendered: {output_path}")

def main():
    """Main function to generate all base buildings"""
    # Setup scene
    setup_isometric_camera()
    setup_simple_lighting()
    setup_fast_render()
    
    # Building types and their relative sizes
    building_types = [
        'command_center',
        'barracks',
        'factory',
        'power_plant'
    ]
    
    for building_type in building_types:
        # Clear scene for new model
        bpy.ops.object.select_all(action='SELECT')
        bpy.ops.object.delete()
        
        # Create and render the building
        building = create_simple_building(building_type)
        render_8_angles(building, os.path.join(output_dir, building_type))

if __name__ == "__main__":
    main()
