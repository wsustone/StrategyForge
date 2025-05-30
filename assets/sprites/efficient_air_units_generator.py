import bpy
import math
import os
from mathutils import Vector

# Clear the scene
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete()

# Path configuration - use a specific output directory with proper permissions
output_dir = r"C:\Users\Juston\Documents\blender"
if not os.path.exists(output_dir):
    try:
        os.makedirs(output_dir)
    except PermissionError:
        print(f"Permission error creating directory: {output_dir}")
        # Fallback to temp directory
        import tempfile
        output_dir = os.path.join(tempfile.gettempdir(), "strategy_forge_sprites")
        if not os.path.exists(output_dir):
            os.makedirs(output_dir)
        print(f"Using temporary directory for sprites: {output_dir}")

def create_simple_unit(unit_type, size=1.0, color=(0.8, 0.2, 0.2, 1.0)):
    """Create a simple unit using basic shapes"""
    bpy.ops.object.select_all(action='DESELECT')
    
    if unit_type == 'fighter':
        # Simple cone pointing forward
        bpy.ops.mesh.primitive_cone_add(vertices=8, radius1=0.5*size, depth=1.5*size)
        unit = bpy.context.active_object
        unit.rotation_euler = (math.radians(90), 0, 0)  # Point along X-axis
        unit.scale = (1, 1, 0.5)  # Flatten slightly
        color = (0.8, 0.2, 0.2, 1.0)  # Red
        
    elif unit_type == 'bomber':
        # Simple box shape
        bpy.ops.mesh.primitive_cube_add(size=size)
        unit = bpy.context.active_object
        unit.scale = (1.5, 2.0, 0.5)
        color = (0.2, 0.2, 0.8, 1.0)  # Blue
        
    elif unit_type == 'large_aircraft':
        # Simple cylinder for large aircraft
        bpy.ops.mesh.primitive_cylinder_add(vertices=12, radius=size, depth=2*size)
        unit = bpy.context.active_object
        unit.scale = (1, 1, 0.3)  # Flatten
        color = (0.2, 0.8, 0.2, 1.0)  # Green
    
    # Set material
    mat = bpy.data.materials.new(name=f"{unit_type}_material")
    mat.diffuse_color = color
    if unit.data.materials:
        unit.data.materials[0] = mat
    else:
        unit.data.materials.append(mat)
    
    # Add an arrow to show forward direction
    bpy.ops.mesh.primitive_cone_add(vertices=8, radius1=0.1, depth=0.5)
    arrow = bpy.context.active_object
    arrow.location = (0.7 * size, 0, 0)  # Position at front
    arrow.rotation_euler = (0, math.radians(90), 0)
    
    # Make arrow a child of unit
    arrow.parent = unit
    
    # Set arrow material (slightly brighter)
    arrow_mat = bpy.data.materials.new(name=f"{unit_type}_arrow_material")
    arrow_mat.diffuse_color = tuple(min(c + 0.3, 1.0) for c in color[:3]) + (1.0,)
    if arrow.data.materials:
        arrow.data.materials[0] = arrow_mat
    else:
        arrow.data.materials.append(arrow_mat)
    
    # Select both objects and join them
    bpy.ops.object.select_all(action='DESELECT')
    unit.select_set(True)
    arrow.select_set(True)
    bpy.context.view_layer.objects.active = unit
    bpy.ops.object.join()
    
    # Name the unit
    unit.name = f"{unit_type.capitalize()}"
    
    return unit





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
    
    # Bloom not available in this Blender version

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
    """Main function to generate all air units"""
    # Setup scene
    setup_isometric_camera()
    setup_simple_lighting()
    setup_fast_render()
    
    # Unit types and their relative sizes
    unit_types = [
        ('fighter', 1.0),
        ('bomber', 1.5),
        ('large_aircraft', 2.0)
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
