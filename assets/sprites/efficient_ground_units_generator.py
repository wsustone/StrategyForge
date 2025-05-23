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

def create_simple_unit(unit_type, size=1.0, color=(0.8, 0.2, 0.2, 1.0)):
    """Create a simple ground unit using basic shapes"""
    bpy.ops.object.select_all(action='DESELECT')
    
    if unit_type == 'tank':
        # Simple tank shape (box with cylinder turret)
        bpy.ops.mesh.primitive_cube_add(size=size)
        body = bpy.context.active_object
        body.scale = (1.5, 2.0, 0.6)
        body.location.z = 0.3
        
        # Turret
        bpy.ops.mesh.primitive_cylinder_add(vertices=8, radius=0.6, depth=0.4)
        turret = bpy.context.active_object
        turret.location = (0, 0, 0.7)
        
        # Gun barrel
        bpy.ops.mesh.primitive_cylinder_add(vertices=6, radius=0.1, depth=1.2)
        barrel = bpy.context.active_object
        barrel.location = (0, 0.8, 0.7)
        barrel.rotation_euler = (math.radians(90), 0, 0)
        
        color = (0.6, 0.6, 0.2, 1.0)  # Olive drab
        
    elif unit_type == 'artillery':
        # Simple artillery shape (box with long barrel)
        bpy.ops.mesh.primitive_cube_add(size=size)
        body = bpy.context.active_object
        body.scale = (1.5, 1.5, 0.5)
        
        # Barrel
        bpy.ops.mesh.primitive_cylinder_add(vertices=8, radius=0.2, depth=2.0)
        barrel = bpy.context.active_object
        barrel.location = (0, 0.8, 0.3)
        barrel.rotation_euler = (math.radians(15), 0, 0)
        
        color = (0.4, 0.4, 0.4, 1.0)  # Gray
        
    elif unit_type == 'harvester':
        # Simple harvester shape (box with scoop)
        bpy.ops.mesh.primitive_cube_add(size=size)
        body = bpy.context.active_object
        body.scale = (1.2, 1.8, 0.8)
        
        # Scoop
        bpy.ops.mesh.primitive_cone_add(vertices=8, radius1=0.8, radius2=0.1, depth=1.0)
        scoop = bpy.context.active_object
        scoop.location = (0, -1.2, 0.2)
        scoop.rotation_euler = (math.radians(90), 0, 0)
        
        color = (0.8, 0.5, 0.1, 1.0)  # Orange
    
    # Set material for all objects
    for obj in bpy.context.selected_objects:
        mat = bpy.data.materials.new(name=f"{unit_type}_material")
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
    
    # Add forward arrow
    bpy.ops.mesh.primitive_cone_add(vertices=8, radius1=0.2, depth=0.5)
    arrow = bpy.context.active_object
    arrow.location = (0, 1.2 * size, 0.4)
    arrow.rotation_euler = (0, math.radians(-90), 0)
    
    # Set arrow material (slightly brighter)
    arrow_mat = bpy.data.materials.new(name=f"{unit_type}_arrow_material")
    arrow_mat.diffuse_color = tuple(min(c + 0.3, 1.0) for c in color[:3]) + (1.0,)
    if arrow.data.materials:
        arrow.data.materials[0] = arrow_mat
    else:
        arrow.data.materials.append(arrow_mat)
    
    # Join arrow with unit
    bpy.ops.object.select_all(action='DESELECT')
    for obj in bpy.data.objects:
        if obj.type == 'MESH':
            obj.select_set(True)
    bpy.context.view_layer.objects.active = bpy.context.selected_objects[0]
    bpy.ops.object.join()
    
    # Name the unit
    bpy.context.active_object.name = f"{unit_type.capitalize()}"
    
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
    
    # Set isometric angle (matching the artillery generator)
    camera.rotation_euler = (math.radians(54.736), 0, math.radians(45))
    
    # Position the camera (matching the artillery generator)
    camera.location = (8, -8, 8)
    
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
    """Main function to generate all ground units"""
    # Setup scene
    setup_isometric_camera()
    setup_simple_lighting()
    setup_fast_render()
    
    # Unit types and their relative sizes
    unit_types = [
        ('tank', 1.0),
        ('artillery', 1.2),
        ('harvester', 1.1)
    ]
    
    for unit_type, size in unit_types:
        # Clear scene for new model
        bpy.ops.object.select_all(action='SELECT')
        bpy.ops.object.delete()
        
        # Create and render the unit
        unit = create_simple_unit(unit_type, size)
        render_8_angles(unit, os.path.join(output_dir, unit_type))

if __name__ == "__main__":
    main()
