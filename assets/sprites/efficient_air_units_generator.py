import bpy
import math
import os

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

def create_fighter():
    """Create an optimized fighter aircraft model"""
    name = "Fighter"
    
    # Create main body (fuselage) - simplified with a cube
    bpy.ops.mesh.primitive_cube_add(size=1.0)
    fuselage = bpy.context.active_object
    fuselage.name = f"{name}_Fuselage"
    fuselage.scale = (1.5, 0.3, 0.3)
    fuselage.location = (0, 0, 0.5)
    
    # Create wings (single plane for efficiency)
    bpy.ops.mesh.primitive_plane_add(size=1.0)
    wings = bpy.context.active_object
    wings.name = f"{name}_Wings"
    wings.scale = (1.0, 2.0, 1.0)
    wings.location = (0, 0, 0.5)
    
    # Create tail (simple vertical stabilizer)
    bpy.ops.mesh.primitive_plane_add(size=0.5)
    tail = bpy.context.active_object
    tail.name = f"{name}_Tail"
    tail.scale = (0.3, 0.3, 0.5)
    tail.location = (-1.2, 0, 0.7)
    tail.rotation_euler = (math.radians(90), 0, 0)
    
    # Create cockpit (simple cube)
    bpy.ops.mesh.primitive_cube_add(size=0.5)
    cockpit = bpy.context.active_object
    cockpit.name = f"{name}_Cockpit"
    cockpit.scale = (0.3, 0.4, 0.2)
    cockpit.location = (0.3, 0, 0.65)
    
    # Select all parts and join them
    bpy.ops.object.select_all(action='DESELECT')
    for obj in [fuselage, wings, tail, cockpit]:
        obj.select_set(True)
    bpy.context.view_layer.objects.active = fuselage
    bpy.ops.object.join()
    
    # Set material
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.diffuse_color = (0.4, 0.4, 0.5, 1.0)  # Gray-blue color
    fuselage.data.materials.append(mat)
    
    return fuselage

def create_bomber():
    """Create an optimized bomber aircraft model"""
    name = "Bomber"
    
    # Create main body (wider and flatter than fighter)
    bpy.ops.mesh.primitive_cube_add(size=1.0)
    body = bpy.context.active_object
    body.name = f"{name}_Body"
    body.scale = (1.8, 0.5, 0.25)
    body.location = (0, 0, 0.6)
    
    # Create wings (wider than fighter)
    bpy.ops.mesh.primitive_plane_add(size=1.0)
    wings = bpy.context.active_object
    wings.name = f"{name}_Wings"
    wings.scale = (1.2, 2.5, 1.0)
    wings.location = (0, 0, 0.6)
    
    # Create tail (double tail for bomber)
    for i in [-1, 1]:
        bpy.ops.mesh.primitive_plane_add(size=0.5)
        tail = bpy.context.active_object
        tail.name = f"{name}_Tail_{i}"
        tail.scale = (0.2, 0.2, 0.4)
        tail.location = (-1.4, 0.3 * i, 0.75)
        tail.rotation_euler = (math.radians(90), 0, 0)
    
    # Create bomb bay (simple cube)
    bpy.ops.mesh.primitive_cube_add(size=0.5)
    bomb_bay = bpy.context.active_object
    bomb_bay.name = f"{name}_BombBay"
    bomb_bay.scale = (0.8, 0.8, 0.2)
    bomb_bay.location = (0, 0, 0.4)
    
    # Select all parts and join them
    bpy.ops.object.select_all(action='DESELECT')
    for obj in bpy.data.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    bpy.context.view_layer.objects.active = body
    bpy.ops.object.join()
    
    # Set material
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.diffuse_color = (0.3, 0.3, 0.35, 1.0)  # Dark gray color
    body.data.materials.append(mat)
    
    return body

def create_large_aircraft():
    """Create an optimized large aircraft model"""
    name = "LargeAircraft"
    
    # Create main body (large and flat)
    bpy.ops.mesh.primitive_cube_add(size=1.0)
    body = bpy.context.active_object
    body.name = f"{name}_Body"
    body.scale = (3.0, 1.0, 0.4)
    body.location = (0, 0, 1.0)
    
    # Create main wings (very wide)
    bpy.ops.mesh.primitive_plane_add(size=1.0)
    wings = bpy.context.active_object
    wings.name = f"{name}_Wings"
    wings.scale = (2.0, 4.0, 1.0)
    wings.location = (0, 0, 1.0)
    
    # Create tail section
    bpy.ops.mesh.primitive_cube_add(size=0.5)
    tail = bpy.context.active_object
    tail.name = f"{name}_Tail"
    tail.scale = (0.6, 0.3, 0.8)
    tail.location = (-2.5, 0, 1.2)
    
    # Create engines (simple cubes)
    for i in [-1, 1]:
        bpy.ops.mesh.primitive_cylinder_add(vertices=8, radius=0.3, depth=0.8)
        engine = bpy.context.active_object
        engine.name = f"{name}_Engine_{i}"
        engine.location = (1.0, 1.2 * i, 0.8)
        engine.rotation_euler = (0, math.radians(90), 0)
    
    # Select all parts and join them
    bpy.ops.object.select_all(action='DESELECT')
    for obj in bpy.data.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    bpy.context.view_layer.objects.active = body
    bpy.ops.object.join()
    
    # Set material
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.diffuse_color = (0.35, 0.35, 0.4, 1.0)  # Bluish-gray color
    body.data.materials.append(mat)
    
    return body

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
    
    # Create and render fighter
    fighter = create_fighter()
    render_8_angles(fighter, os.path.join(output_dir, "fighter"))
    
    # Clear scene for next model
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete()
    
    # Create and render bomber
    bomber = create_bomber()
    render_8_angles(bomber, os.path.join(output_dir, "bomber"))
    
    # Clear scene for next model
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete()
    
    # Create and render large aircraft
    large_aircraft = create_large_aircraft()
    render_8_angles(large_aircraft, os.path.join(output_dir, "large_aircraft"))

if __name__ == "__main__":
    main()
