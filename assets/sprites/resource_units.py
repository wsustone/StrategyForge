import bpy
import math
import os
import random

# First, let's make sure we have a clean scene
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete()

# Create resource deposits (wood, stone, iron)
def create_resource_deposit(resource_type="wood"):
    """Create a resource deposit of the specified type"""
    name = f"{resource_type.capitalize()}Deposit"
    
    if resource_type.lower() == "wood":
        # Create a stack of logs
        logs = []
        for i in range(5):
            bpy.ops.mesh.primitive_cylinder_add(
                radius=0.2,
                depth=2.0
            )
            log = bpy.context.active_object
            log.name = f"{name}_Log_{i}"
            
            # Position logs in a stack
            x_offset = random.uniform(-0.2, 0.2)
            z_offset = 0.2 * i
            log.location = (x_offset, 0, z_offset)
            
            # Rotate logs slightly for more natural look
            log.rotation_euler = (
                0, 
                random.uniform(-0.2, 0.2), 
                random.uniform(0, math.pi)
            )
            logs.append(log)
        
        # Create some scattered twigs/branches
        for i in range(3):
            bpy.ops.mesh.primitive_cylinder_add(
                radius=0.1,
                depth=1.0
            )
            twig = bpy.context.active_object
            twig.name = f"{name}_Twig_{i}"
            
            # Position twigs around the stack
            angle = i * (2 * math.pi / 3)
            radius = 0.8
            twig.location = (
                radius * math.cos(angle),
                radius * math.sin(angle),
                0.1
            )
            
            # Random rotation for natural look
            twig.rotation_euler = (
                random.uniform(0, 0.5),
                random.uniform(0, 0.5),
                angle
            )
        
        # Create a simple material
        mat = bpy.data.materials.new(name=f"{name}_Material")
        mat.use_nodes = True
        principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
        if principled_bsdf:
            principled_bsdf.inputs[0].default_value = (0.35, 0.2, 0.05, 1.0)  # Brown
        
    elif resource_type.lower() == "stone":
        # Create a pile of rocks
        rocks = []
        for i in range(7):
            # Create a basic rock shape
            bpy.ops.mesh.primitive_ico_sphere_add(
                radius=0.3 + random.uniform(-0.1, 0.1),
                subdivisions=2
            )
            rock = bpy.context.active_object
            rock.name = f"{name}_Rock_{i}"
            
            # Randomize the shape a bit
            for v in rock.data.vertices:
                v.co.x += random.uniform(-0.05, 0.05)
                v.co.y += random.uniform(-0.05, 0.05)
                v.co.z += random.uniform(-0.05, 0.05)
            
            # Position in a pile
            angle = i * (2 * math.pi / 7)
            radius = 0.4 if i > 0 else 0
            height = 0.2 if i > 3 else 0
            rock.location = (
                radius * math.cos(angle),
                radius * math.sin(angle),
                height
            )
            
            # Random rotation
            rock.rotation_euler = (
                random.uniform(0, math.pi),
                random.uniform(0, math.pi),
                random.uniform(0, math.pi)
            )
            
            # Random scale
            scale_factor = 0.8 + random.uniform(0, 0.4)
            rock.scale = (scale_factor, scale_factor, scale_factor)
            rocks.append(rock)
        
        # Create a simple material
        mat = bpy.data.materials.new(name=f"{name}_Material")
        mat.use_nodes = True
        principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
        if principled_bsdf:
            principled_bsdf.inputs[0].default_value = (0.4, 0.4, 0.4, 1.0)  # Gray
    
    elif resource_type.lower() == "iron":
        # Create ore deposit
        bpy.ops.mesh.primitive_cube_add(size=1.0)
        base = bpy.context.active_object
        base.name = f"{name}_Base"
        base.location = (0, 0, 0.25)
        base.scale = (1.0, 1.0, 0.5)
        
        # Add ore protrusions
        for i in range(5):
            bpy.ops.mesh.primitive_cube_add(size=0.4)
            ore = bpy.context.active_object
            ore.name = f"{name}_Ore_{i}"
            
            # Position ores on top of base
            angle = i * (2 * math.pi / 5)
            radius = 0.4
            ore.location = (
                radius * math.cos(angle),
                radius * math.sin(angle),
                0.6
            )
            
            # Random rotation
            ore.rotation_euler = (
                random.uniform(0, 0.5),
                random.uniform(0, 0.5),
                random.uniform(0, math.pi/2)
            )
            
            # Random scale for variety
            ore.scale = (
                0.7 + random.uniform(0, 0.5),
                0.7 + random.uniform(0, 0.5),
                0.7 + random.uniform(0, 0.5)
            )
        
        # Create a simple material
        mat = bpy.data.materials.new(name=f"{name}_Material")
        mat.use_nodes = True
        principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
        if principled_bsdf:
            principled_bsdf.inputs[0].default_value = (0.2, 0.05, 0.05, 1.0)  # Rusty red
            principled_bsdf.inputs[4].default_value = 0.8  # Metallic
    
    # Join all parts
    for obj in bpy.context.scene.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    
    if bpy.context.selected_objects:
        bpy.context.view_layer.objects.active = bpy.context.selected_objects[0]
        bpy.ops.object.join()
        
    # Get the joined object
    deposit = bpy.context.active_object
    deposit.name = name
    
    # Apply material
    if deposit.data.materials:
        deposit.data.materials[0] = mat
    else:
        deposit.data.materials.append(mat)
    
    return deposit

# Create gatherer unit (worker that collects resources)
def create_gatherer():
    """Create a steampunk-style gatherer unit"""
    name = "Gatherer"
    
    # Create torso
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.3,
        depth=0.6
    )
    torso = bpy.context.active_object
    torso.name = f"{name}_Torso"
    torso.location = (0, 0, 0.6)
    
    # Create head
    bpy.ops.mesh.primitive_uv_sphere_add(
        radius=0.2
    )
    head = bpy.context.active_object
    head.name = f"{name}_Head"
    head.location = (0, 0, 1.1)
    
    # Create hat/helmet
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.22,
        depth=0.15
    )
    helmet = bpy.context.active_object
    helmet.name = f"{name}_Helmet"
    helmet.location = (0, 0, 1.25)
    
    # Create a small lamp on the helmet (miner's lamp)
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.05,
        depth=0.1
    )
    lamp = bpy.context.active_object
    lamp.name = f"{name}_Lamp"
    lamp.location = (0, 0.22, 1.25)
    lamp.rotation_euler = (math.radians(90), 0, 0)
    
    # Create legs
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.1,
            depth=0.5
        )
        leg = bpy.context.active_object
        leg.name = f"{name}_Leg_{side}"
        leg.location = (side * 0.2, 0, 0.3)
        
    # Create arms
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.08,
            depth=0.4
        )
        arm = bpy.context.active_object
        arm.name = f"{name}_Arm_{side}"
        arm.rotation_euler = (0, math.radians(90), 0)
        arm.location = (side * 0.35, 0, 0.8)
    
    # Create gathering tool (pickaxe)
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.05,
        depth=0.6
    )
    handle = bpy.context.active_object
    handle.name = f"{name}_PickHandle"
    handle.rotation_euler = (math.radians(45), 0, 0)
    handle.location = (0.4, 0.3, 0.8)
    
    # Create pickaxe head
    bpy.ops.mesh.primitive_cone_add(
        radius1=0.1,
        radius2=0,
        depth=0.2
    )
    pick = bpy.context.active_object
    pick.name = f"{name}_PickHead"
    pick.rotation_euler = (0, math.radians(90), 0)
    pick.location = (0.4, 0.6, 0.9)
    
    # Create a backpack for storing resources
    bpy.ops.mesh.primitive_cube_add(size=0.4)
    backpack = bpy.context.active_object
    backpack.name = f"{name}_Backpack"
    backpack.scale = (0.6, 0.3, 0.8)
    backpack.location = (0, -0.25, 0.7)
    
    # Create straps for backpack
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cube_add(size=0.1)
        strap = bpy.context.active_object
        strap.name = f"{name}_Strap_{side}"
        strap.scale = (0.2, 2.0, 0.5)
        strap.location = (side * 0.15, -0.15, 0.7)
    
    # Create a simple material
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.use_nodes = True
    principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
    if principled_bsdf:
        principled_bsdf.inputs[0].default_value = (0.3, 0.25, 0.2, 1.0)  # Brown-ish
    
    # Join all parts
    for obj in bpy.context.scene.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    
    if bpy.context.selected_objects:
        bpy.context.view_layer.objects.active = bpy.context.selected_objects[0]
        bpy.ops.object.join()
        
    # Get the joined object
    gatherer = bpy.context.active_object
    gatherer.name = name
    
    # Apply material
    if gatherer.data.materials:
        gatherer.data.materials[0] = mat
    else:
        gatherer.data.materials.append(mat)
    
    return gatherer

# Create engineer unit (builds and repairs structures)
def create_engineer():
    """Create a steampunk-style engineer unit"""
    name = "Engineer"
    
    # Create torso (slightly larger than gatherer)
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.35,
        depth=0.7
    )
    torso = bpy.context.active_object
    torso.name = f"{name}_Torso"
    torso.location = (0, 0, 0.65)
    
    # Create head
    bpy.ops.mesh.primitive_uv_sphere_add(
        radius=0.22
    )
    head = bpy.context.active_object
    head.name = f"{name}_Head"
    head.location = (0, 0, 1.15)
    
    # Create engineer's cap
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.25,
        depth=0.2,
        vertices=8
    )
    cap = bpy.context.active_object
    cap.name = f"{name}_Cap"
    cap.location = (0, 0, 1.3)
    
    # Create goggles
    bpy.ops.mesh.primitive_torus_add(
        major_radius=0.1,
        minor_radius=0.03
    )
    goggles = bpy.context.active_object
    goggles.name = f"{name}_Goggles"
    goggles.location = (0, 0.15, 1.15)
    goggles.rotation_euler = (math.radians(90), 0, 0)
    
    # Create legs
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.12,
            depth=0.6
        )
        leg = bpy.context.active_object
        leg.name = f"{name}_Leg_{side}"
        leg.location = (side * 0.2, 0, 0.3)
        
    # Create arms
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.1,
            depth=0.5
        )
        arm = bpy.context.active_object
        arm.name = f"{name}_Arm_{side}"
        arm.rotation_euler = (0, math.radians(90), 0)
        arm.location = (side * 0.4, 0, 0.85)
    
    # Create tool belt
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.4,
        depth=0.1
    )
    belt = bpy.context.active_object
    belt.name = f"{name}_ToolBelt"
    belt.location = (0, 0, 0.5)
    
    # Create tools on belt
    for i in range(4):
        angle = i * (math.pi / 2)
        # Create a tool
        bpy.ops.mesh.primitive_cube_add(size=0.1)
        tool = bpy.context.active_object
        tool.name = f"{name}_Tool_{i}"
        tool.scale = (0.5, 0.2, 1.0)
        tool.location = (
            0.3 * math.cos(angle),
            0.3 * math.sin(angle),
            0.5
        )
        tool.rotation_euler = (0, 0, angle)
    
    # Create wrench (held in one hand)
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.05,
        depth=0.3
    )
    wrench_handle = bpy.context.active_object
    wrench_handle.name = f"{name}_WrenchHandle"
    wrench_handle.location = (0.55, 0.1, 0.85)
    
    bpy.ops.mesh.primitive_torus_add(
        major_radius=0.1,
        minor_radius=0.03,
        major_segments=6
    )
    wrench_head = bpy.context.active_object
    wrench_head.name = f"{name}_WrenchHead"
    wrench_head.location = (0.55, 0.3, 0.85)
    
    # Create blueprint (held in other hand)
    bpy.ops.mesh.primitive_cube_add(size=0.1)
    blueprint = bpy.context.active_object
    blueprint.name = f"{name}_Blueprint"
    blueprint.scale = (0.3, 0.4, 0.01)
    blueprint.location = (-0.55, 0.2, 0.85)
    
    # Create a simple material
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.use_nodes = True
    principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
    if principled_bsdf:
        principled_bsdf.inputs[0].default_value = (0.2, 0.2, 0.3, 1.0)  # Blue-ish gray
    
    # Join all parts
    for obj in bpy.context.scene.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    
    if bpy.context.selected_objects:
        bpy.context.view_layer.objects.active = bpy.context.selected_objects[0]
        bpy.ops.object.join()
        
    # Get the joined object
    engineer = bpy.context.active_object
    engineer.name = name
    
    # Apply material
    if engineer.data.materials:
        engineer.data.materials[0] = mat
    else:
        engineer.data.materials.append(mat)
    
    return engineer

# Set up isometric camera
def setup_isometric_camera():
    bpy.ops.object.camera_add()
    camera = bpy.context.active_object
    camera.name = 'IsometricCamera'
    
    # True isometric angle is approximately 54.736° from horizontal
    camera.rotation_euler = (math.radians(54.736), 0, math.radians(45))
    
    # Place camera at a good distance
    camera.location = (10, -10, 10)
    
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
    key_light.location = (5, -2, 8)
    key_light.rotation_euler = (math.radians(60), 0, math.radians(20))
    key_light.data.energy = 800
    
    # Fill light (softer light from opposite side)
    bpy.ops.object.light_add(type='AREA')
    fill_light = bpy.context.active_object
    fill_light.name = 'FillLight'
    fill_light.location = (-5, 2, 5)
    fill_light.rotation_euler = (math.radians(45), 0, math.radians(-20))
    fill_light.data.energy = 400
    
    # Back light (rim light)
    bpy.ops.object.light_add(type='AREA')
    back_light = bpy.context.active_object
    back_light.name = 'BackLight'
    back_light.location = (0, 5, 4)
    back_light.rotation_euler = (math.radians(30), math.radians(-20), 0)
    back_light.data.energy = 600

# Setup render settings
def setup_render(resolution_x=512, resolution_y=512):
    # Set render engine to Cycles or Eevee
    bpy.context.scene.render.engine = 'CYCLES'  # 'BLENDER_EEVEE' is also a good option for faster renders
    
    # Set render settings
    bpy.context.scene.render.resolution_x = resolution_x
    bpy.context.scene.render.resolution_y = resolution_y
    bpy.context.scene.render.resolution_percentage = 100
    bpy.context.scene.render.film_transparent = True  # Transparent background
    
    # Use a specific output directory for all sprites
    output_dir = r"C:\Users\Juston\Documents\blender"
    
    # Make sure directory exists
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
    
    print(f"Sprites will be saved to: {output_dir}")
    
    return output_dir

# Render the object from multiple angles
def render_object_rotations(obj, output_dir, angles=None):
    if angles is None:
        # Default to 8 angles (45° increments) for more directional options
        angles = [0, 45, 90, 135, 180, 225, 270, 315]
    
    original_rotation = obj.rotation_euler.copy()
    
    for angle in angles:
        # Rotate the object
        obj.rotation_euler.z = math.radians(angle)
        
        # Set output path
        bpy.context.scene.render.filepath = os.path.join(output_dir, f"{obj.name}_rot{angle}.png")
        
        # Render
        bpy.ops.render.render(write_still=True)
        
        print(f"Rendered {obj.name} at {angle}° rotation")
    
    # Reset rotation
    obj.rotation_euler = original_rotation

# Main execution
def main():
    # Setup the scene
    output_dir = setup_render()
    setup_isometric_camera()
    setup_lighting()
    
    # Ask user what to create
    print("\n=== StrategyForge Resource Unit Generator ===")
    print("1. Create and render Wood deposit")
    print("2. Create and render Stone deposit")
    print("3. Create and render Iron deposit")
    print("4. Create and render Gatherer unit")
    print("5. Create and render Engineer unit")
    print("6. Create and render all")
    print("7. Exit")
    
    choice = input("Enter your choice (1-7): ")
    
    if choice == "1":
        deposit = create_resource_deposit("wood")
        render_object_rotations(deposit, output_dir, [0])  # Single angle for deposits
    elif choice == "2":
        deposit = create_resource_deposit("stone")
        render_object_rotations(deposit, output_dir, [0])  # Single angle for deposits
    elif choice == "3":
        deposit = create_resource_deposit("iron")
        render_object_rotations(deposit, output_dir, [0])  # Single angle for deposits
    elif choice == "4":
        gatherer = create_gatherer()
        render_object_rotations(gatherer, output_dir)
    elif choice == "5":
        engineer = create_engineer()
        render_object_rotations(engineer, output_dir)
    elif choice == "6":
        # Create and render all resource units
        wood = create_resource_deposit("wood")
        render_object_rotations(wood, output_dir, [0])
        
        # Clear scene for next object
        bpy.ops.object.select_all(action='SELECT')
        bpy.ops.object.delete()
        
        stone = create_resource_deposit("stone")
        render_object_rotations(stone, output_dir, [0])
        
        # Clear scene for next object
        bpy.ops.object.select_all(action='SELECT')
        bpy.ops.object.delete()
        
        iron = create_resource_deposit("iron")
        render_object_rotations(iron, output_dir, [0])
        
        # Clear scene for next object
        bpy.ops.object.select_all(action='SELECT')
        bpy.ops.object.delete()
        
        gatherer = create_gatherer()
        render_object_rotations(gatherer, output_dir)
        
        # Clear scene for next object
        bpy.ops.object.select_all(action='SELECT')
        bpy.ops.object.delete()
        
        engineer = create_engineer()
        render_object_rotations(engineer, output_dir)
    else:
        print("Exiting...")
    
    print(f"Finished rendering to {output_dir}")

# Run the script
if __name__ == "__main__":
    main()
