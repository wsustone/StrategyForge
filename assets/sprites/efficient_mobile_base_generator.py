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

# Optimized mobile base model creation
def create_simple_mobile_base(name="MobileBase", color=(0.75, 0.65, 0.5, 1.0)):
    """Create a simplified steampunk mobile base model that's efficient to render"""
    # Create base platform
    bpy.ops.mesh.primitive_cube_add(size=1)
    base = bpy.context.active_object
    base.name = f"{name}_Base"
    base.scale = (4.0, 4.0, 0.5)
    base.location = (0, 0, 0.25)
    
    # Create main dome/structure
    bpy.ops.mesh.primitive_uv_sphere_add(
        radius=2.0,
        segments=16,  # Reduced segments for better performance
        ring_count=8  # Reduced rings for better performance
    )
    dome = bpy.context.active_object
    dome.name = f"{name}_Dome"
    dome.scale = (1.0, 1.0, 0.6)
    dome.location = (0, 0, 2.0)
    
    # Create smokestack/chimney
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.4,
        depth=2.0,
        vertices=8  # Reduced vertices for better performance
    )
    chimney = bpy.context.active_object
    chimney.name = f"{name}_Chimney"
    chimney.location = (1.5, 1.5, 2.0)
    
    # Create simplified tracks/wheels (4 corner blocks)
    for x_pos in [-1.8, 1.8]:
        for y_pos in [-1.8, 1.8]:
            bpy.ops.mesh.primitive_cube_add(size=1)
            track = bpy.context.active_object
            track.name = f"{name}_Track_{x_pos}_{y_pos}"
            track.scale = (0.7, 0.7, 0.3)
            track.location = (x_pos, y_pos, 0.0)
    
    # Create a simple antenna/mast
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.1,
        depth=3.0,
        vertices=6  # Reduced vertices for better performance
    )
    antenna = bpy.context.active_object
    antenna.name = f"{name}_Antenna"
    antenna.location = (-1.0, 0, 3.0)
    
    # Create a material for the mobile base
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.use_nodes = True
    
    # Set base color
    principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
    if principled_bsdf:
        principled_bsdf.inputs[0].default_value = color
        principled_bsdf.inputs[4].default_value = 0.7  # Metallic
        principled_bsdf.inputs[7].default_value = 0.3  # Roughness
    
    # Apply material to all objects
    for obj in bpy.context.selected_objects:
        if obj.data.materials:
            obj.data.materials[0] = mat
        else:
            obj.data.materials.append(mat)
    
    # Select all objects and join them
    bpy.ops.object.select_all(action='DESELECT')
    for obj in bpy.data.objects:
        if name in obj.name:
            obj.select_set(True)
    
    bpy.context.view_layer.objects.active = base
    bpy.ops.object.join()
    
    # Return the final joined object
    return bpy.context.active_object

# Setup an optimized camera with greater distance for the larger model
def setup_isometric_camera():
    """Set up an isometric camera"""
    bpy.ops.object.camera_add()
    camera = bpy.context.active_object
    camera.name = 'IsometricCamera'
    
    # Set isometric angle
    camera.rotation_euler = (math.radians(54.736), 0, math.radians(45))
    
    # Position the camera - further back for the larger base
    camera.location = (15, -15, 15)
    
    # Set as active camera
    bpy.context.scene.camera = camera
    
    return camera

# Setup simple lighting
def setup_simple_lighting():
    """Create a simplified lighting setup for faster renders"""
    # Add a key light
    bpy.ops.object.light_add(type='SUN')
    key_light = bpy.context.active_object
    key_light.name = 'KeyLight'
    key_light.rotation_euler = (math.radians(45), 0, math.radians(45))
    key_light.data.energy = 2.0
    
    # Add simple fill light
    bpy.ops.object.light_add(type='SUN')
    fill_light = bpy.context.active_object
    fill_light.name = 'FillLight'
    fill_light.rotation_euler = (math.radians(45), 0, math.radians(-45))
    fill_light.data.energy = 1.0

# Setup optimized render settings for speed
def setup_fast_render():
    """Configure render settings for speed"""
    # Use Eevee Next or fallback to Cycles for rendering
    try:
        bpy.context.scene.render.engine = 'BLENDER_EEVEE_NEXT'
        print("Using BLENDER_EEVEE_NEXT renderer")
    except:
        # If EEVEE_NEXT fails, try Cycles
        bpy.context.scene.render.engine = 'CYCLES'
        print("Falling back to CYCLES renderer")
    
    # Set render settings - reduced resolution for speed
    bpy.context.scene.render.resolution_x = 512  # Keeping higher resolution for the base
    bpy.context.scene.render.resolution_y = 512
    bpy.context.scene.render.resolution_percentage = 100
    bpy.context.scene.render.film_transparent = True  # Transparent background
    
    # Optimize Eevee settings for Blender 4.4
    try:
        # For EEVEE_NEXT
        if bpy.context.scene.render.engine == 'BLENDER_EEVEE_NEXT':
            # Access Eevee Next settings
            bpy.context.scene.eevee_next.render_samples = 16  # Lower sample count
            bpy.context.scene.eevee_next.ambient_occlusion.enabled = False
            bpy.context.scene.eevee_next.screen_space_reflection.enabled = False
            bpy.context.scene.eevee_next.bloom.enabled = False
        # For Cycles
        elif bpy.context.scene.render.engine == 'CYCLES':
            bpy.context.scene.cycles.samples = 32  # Lower sample count
            bpy.context.scene.cycles.use_adaptive_sampling = True
            bpy.context.scene.cycles.time_limit = 30  # 30 second render time limit
    except AttributeError:
        print("Some render settings couldn't be applied - this is normal for different Blender versions")

# Render 8 angles of the object
def render_8_angles(obj, output_dir):
    """Render an object from 8 angles (45° increments)"""
    angles = [0, 45, 90, 135, 180, 225, 270, 315]
    
    for angle in angles:
        # Rotate the object
        obj.rotation_euler.z = math.radians(angle)
        
        # Set output path with cleaner naming (without _Body, _Base, etc.)
        clean_name = "SteampunkBase"
        angle_output_file = os.path.join(output_dir, f"{clean_name}_rot{angle}.png")
        bpy.context.scene.render.filepath = angle_output_file
        
        # Render
        bpy.ops.render.render(write_still=True)
        print(f"Rendered {obj.name} at {angle}° rotation: {angle_output_file}")

def main():
    """Main function to run the script"""
    # Set up the scene
    setup_isometric_camera()
    setup_simple_lighting()
    setup_fast_render()
    
    # Create mobile base with a brass/copper color
    mobile_base = create_simple_mobile_base(name="SteampunkBase", color=(0.75, 0.65, 0.5, 1.0))
    
    # Render from 8 angles
    render_8_angles(mobile_base, output_dir)
    
    print(f"All renders completed. Files saved to: {output_dir}")

if __name__ == "__main__":
    main()
