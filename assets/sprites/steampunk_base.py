import bpy
import math
import os
import random

# First, let's make sure we have a clean scene
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete()

# Create a steampunk mobile base
def create_steampunk_base(name="MobileBase"):
    # Scale factor compared to normal units
    scale_factor = 4.0
    
    # Create base platform (main hull)
    bpy.ops.mesh.primitive_cube_add(size=1.5 * scale_factor)
    base = bpy.context.active_object
    base.name = f"{name}_Base"
    base.scale = (1.0, 1.5, 0.3)
    base.location = (0, 0, 0.3 * scale_factor)
    
    # Create tracks (larger and more industrial looking)
    for side in [-1, 1]:
        # Main track body
        bpy.ops.mesh.primitive_cube_add(size=0.5 * scale_factor)
        track = bpy.context.active_object
        track.name = f"{name}_Track_{side}"
        track.scale = (1.8, 0.5, 0.25)
        track.location = (side * 1.4 * scale_factor, 0, 0.25 * scale_factor)
        
        # Track details - add gear-like wheels
        for pos in [-1.0, -0.5, 0, 0.5, 1.0]:
            bpy.ops.mesh.primitive_cylinder_add(
                radius=0.2 * scale_factor,
                depth=0.1 * scale_factor
            )
            wheel = bpy.context.active_object
            wheel.name = f"{name}_Wheel_{side}_{pos}"
            wheel.rotation_euler = (0, math.radians(90), 0)
            wheel.location = (
                side * 1.5 * scale_factor,
                pos * scale_factor,
                0.25 * scale_factor
            )
    
    # Create main structure on the platform
    bpy.ops.mesh.primitive_cube_add(size=1.2 * scale_factor)
    structure = bpy.context.active_object
    structure.name = f"{name}_MainStructure"
    structure.scale = (0.8, 1.2, 0.8)
    structure.location = (0, 0, 1.0 * scale_factor)
    
    # Add smokestacks (steampunk element)
    for side in [-1, 1]:
        for pos in [-0.5, 0.5]:
            # Create main smokestack
            bpy.ops.mesh.primitive_cylinder_add(
                radius=0.15 * scale_factor,
                depth=1.0 * scale_factor
            )
            smokestack = bpy.context.active_object
            smokestack.name = f"{name}_Smokestack_{side}_{pos}"
            smokestack.location = (
                side * 0.5 * scale_factor,
                pos * scale_factor,
                1.8 * scale_factor
            )
            
            # Add cap to smokestack
            bpy.ops.mesh.primitive_cylinder_add(
                radius=0.2 * scale_factor,
                depth=0.1 * scale_factor
            )
            cap = bpy.context.active_object
            cap.name = f"{name}_StackCap_{side}_{pos}"
            cap.location = (
                side * 0.5 * scale_factor,
                pos * scale_factor,
                2.3 * scale_factor
            )
    
    # Add a central command tower
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.4 * scale_factor,
        depth=0.8 * scale_factor
    )
    tower = bpy.context.active_object
    tower.name = f"{name}_CommandTower"
    tower.location = (0, 0, 1.8 * scale_factor)
    
    # Add a dome on top of the tower
    bpy.ops.mesh.primitive_uv_sphere_add(
        radius=0.4 * scale_factor
    )
    dome = bpy.context.active_object
    dome.name = f"{name}_Dome"
    dome.scale = (1.0, 1.0, 0.5)
    dome.location = (0, 0, 2.2 * scale_factor)
    
    # Add gears and mechanical details (steampunk elements)
    for i in range(8):
        angle = i * (math.pi / 4)
        # Create gear
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.2 * scale_factor,
            depth=0.05 * scale_factor,
            vertices=12
        )
        gear = bpy.context.active_object
        gear.name = f"{name}_Gear_{i}"
        
        # Position gears around the structure
        x = math.cos(angle) * 0.9 * scale_factor
        y = math.sin(angle) * 0.9 * scale_factor
        gear.location = (x, y, 1.3 * scale_factor)
        
        # Random rotation for variety
        gear.rotation_euler = (0, 0, random.uniform(0, math.pi*2))
    
    # Add some pipes connecting various parts
    def add_pipe(start, end, radius):
        # Calculate direction vector
        direction = end - start
        length = direction.length
        direction.normalize()
        
        # Create rotation quaternion from direction
        up = mathutils.Vector((0, 0, 1))
        if abs(direction.dot(up)) > 0.999:
            # If direction is parallel to up vector, use a different reference
            reference = mathutils.Vector((1, 0, 0))
        else:
            reference = up
            
        rotation = direction.to_track_quat('Z', 'Y')
        
        # Create pipe
        bpy.ops.mesh.primitive_cylinder_add(
            radius=radius,
            depth=length
        )
        pipe = bpy.context.active_object
        
        # Position and rotate pipe
        pipe.rotation_mode = 'QUATERNION'
        pipe.rotation_quaternion = rotation
        
        # Position pipe at midpoint and adjust for length
        pipe.location = start + direction * (length / 2)
        
        return pipe
    
    # Add some decorative piping (steampunk element)
    for i in range(6):
        # Create random pipe endpoints on the structure
        x1 = random.uniform(-0.7, 0.7) * scale_factor
        y1 = random.uniform(-1.0, 1.0) * scale_factor
        z1 = random.uniform(0.8, 1.5) * scale_factor
        
        x2 = random.uniform(-0.7, 0.7) * scale_factor
        y2 = random.uniform(-1.0, 1.0) * scale_factor
        z2 = random.uniform(0.8, 1.5) * scale_factor
        
        # Create simple bent pipe (two segments)
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.05 * scale_factor,
            depth=0.5 * scale_factor
        )
        pipe = bpy.context.active_object
        pipe.name = f"{name}_Pipe_{i}"
        pipe.location = (x1, y1, z1)
        
        # Create pipe joints (spheres)
        bpy.ops.mesh.primitive_uv_sphere_add(
            radius=0.07 * scale_factor
        )
        joint = bpy.context.active_object
        joint.name = f"{name}_PipeJoint_{i}"
        joint.location = (x2, y2, z2)
    
    # Add some steam vents
    for i in range(3):
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.08 * scale_factor,
            depth=0.1 * scale_factor
        )
        vent = bpy.context.active_object
        vent.name = f"{name}_Vent_{i}"
        
        # Position vents randomly on the upper structure
        vent.location = (
            random.uniform(-0.6, 0.6) * scale_factor,
            random.uniform(-1.0, 1.0) * scale_factor,
            1.6 * scale_factor
        )
    
    # Add a steampunk-style cannon or weapon system
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.2 * scale_factor,
        depth=1.5 * scale_factor
    )
    cannon = bpy.context.active_object
    cannon.name = f"{name}_MainCannon"
    cannon.rotation_euler = (0, math.radians(90), 0)
    cannon.location = (0, 1.2 * scale_factor, 1.0 * scale_factor)
    
    # Add a reinforcement collar for the cannon
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.25 * scale_factor,
        depth=0.2 * scale_factor
    )
    collar = bpy.context.active_object
    collar.name = f"{name}_CannonCollar"
    collar.rotation_euler = (0, math.radians(90), 0)
    collar.location = (0, 0.6 * scale_factor, 1.0 * scale_factor)
    
    # Create material for base
    base_mat = bpy.data.materials.new(name=f"{name}_BaseMaterial")
    base_mat.use_nodes = True
    
    # Set base color to a rusty metal color
    principled_bsdf = base_mat.node_tree.nodes.get('Principled BSDF')
    if principled_bsdf:
        principled_bsdf.inputs[0].default_value = (0.35, 0.25, 0.2, 1.0)  # Rusty brown
        principled_bsdf.inputs[4].default_value = 0.2  # Metallic
        principled_bsdf.inputs[7].default_value = 0.4  # Roughness
    
    # Join all parts
    for obj in bpy.context.scene.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    
    if bpy.context.selected_objects:
        bpy.context.view_layer.objects.active = bpy.context.selected_objects[0]
        bpy.ops.object.join()
        
    # Get the joined object
    base_obj = bpy.context.active_object
    base_obj.name = name
    
    # Apply material
    if base_obj.data.materials:
        base_obj.data.materials[0] = base_mat
    else:
        base_obj.data.materials.append(base_mat)
    
    return base_obj

# Set up isometric camera
def setup_isometric_camera():
    bpy.ops.object.camera_add()
    camera = bpy.context.active_object
    camera.name = 'IsometricCamera'
    
    # True isometric angle is approximately 54.736° from horizontal
    camera.rotation_euler = (math.radians(54.736), 0, math.radians(45))
    
    # Place camera at a good distance for the large base
    camera.location = (20, -20, 20)
    
    # Set camera as active
    bpy.context.scene.camera = camera
    
    return camera

# Set up lighting
def setup_lighting():
    # Create a three-point lighting setup
    
    # Key light (main light)
    bpy.ops.object.light_add(type='AREA')
    key_light = bpy.context.active_object
    key_light.name = 'KeyLight'
    key_light.location = (10, -4, 16)
    key_light.rotation_euler = (math.radians(60), 0, math.radians(20))
    key_light.data.energy = 1000
    
    # Fill light (softer light from opposite side)
    bpy.ops.object.light_add(type='AREA')
    fill_light = bpy.context.active_object
    fill_light.name = 'FillLight'
    fill_light.location = (-10, 4, 10)
    fill_light.rotation_euler = (math.radians(45), 0, math.radians(-20))
    fill_light.data.energy = 600
    
    # Back light (rim light)
    bpy.ops.object.light_add(type='AREA')
    back_light = bpy.context.active_object
    back_light.name = 'BackLight'
    back_light.location = (0, 10, 8)
    back_light.rotation_euler = (math.radians(30), math.radians(-20), 0)
    back_light.data.energy = 800
    
    # Add an additional light for details
    bpy.ops.object.light_add(type='AREA')
    detail_light = bpy.context.active_object
    detail_light.name = 'DetailLight'
    detail_light.location = (0, 0, 15)
    detail_light.rotation_euler = (0, 0, 0)
    detail_light.data.energy = 400

# Setup render settings
def setup_render(resolution_x=1024, resolution_y=1024):
    # Higher resolution for the detailed base
    
    # Set render engine to Cycles or Eevee
    bpy.context.scene.render.engine = 'CYCLES'  # 'BLENDER_EEVEE' is also a good option for faster renders
    
    # Set render settings
    bpy.context.scene.render.resolution_x = resolution_x
    bpy.context.scene.render.resolution_y = resolution_y
    bpy.context.scene.render.resolution_percentage = 100
    bpy.context.scene.render.film_transparent = True  # Transparent background
    
    # Create output directory in a location where the user definitely has write permissions
    # Use a specific path within the StrategyForge project
    output_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), "exported_sprites")
    
    # Make sure directory exists
    if not os.path.exists(output_dir):
        try:
            os.makedirs(output_dir)
        except PermissionError:
            # Fallback to user's documents folder if we can't write to the script directory
            import tempfile
            output_dir = os.path.join(tempfile.gettempdir(), "strategy_forge_sprites")
            if not os.path.exists(output_dir):
                os.makedirs(output_dir)
            print(f"Using temporary directory for sprites: {output_dir}")
    
    return output_dir

# Render the base from multiple angles
def render_base_rotations(base, output_dir, angles=None):
    if angles is None:
        # Default to 8 angles (45° increments) for more directional options
        angles = [0, 45, 90, 135, 180, 225, 270, 315]
    
    original_rotation = base.rotation_euler.copy()
    
    for angle in angles:
        # Rotate the base
        base.rotation_euler.z = math.radians(angle)
        
        # Set output path
        bpy.context.scene.render.filepath = os.path.join(output_dir, f"{base.name}_rot{angle}.png")
        
        # Render
        bpy.ops.render.render(write_still=True)
        
        print(f"Rendered {base.name} at {angle}° rotation")
    
    # Reset rotation
    base.rotation_euler = original_rotation

# Main execution
def main():
    # Import additional modules needed
    global mathutils
    import mathutils
    
    # Setup the scene
    output_dir = setup_render()
    setup_isometric_camera()
    setup_lighting()
    
    # Create the steampunk mobile base
    base = create_steampunk_base("SteampunkBase")
    
    # Render the base from multiple angles
    render_base_rotations(base, output_dir)
    
    print(f"Finished rendering steampunk base sprites to {output_dir}")

# Run the script
if __name__ == "__main__":
    main()
