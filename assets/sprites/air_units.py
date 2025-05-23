import bpy
import math
import os
import random

# First, let's make sure we have a clean scene
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete()

# Create air-to-air fighter
def create_fighter():
    """Create a steampunk-style fighter aircraft"""
    name = "Fighter"
    
    # Create main body (fuselage)
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.3,
        depth=2.0
    )
    fuselage = bpy.context.active_object
    fuselage.name = f"{name}_Fuselage"
    fuselage.rotation_euler = (0, math.radians(90), 0)
    fuselage.location = (0, 0, 0.5)
    
    # Create nose cone
    bpy.ops.mesh.primitive_cone_add(
        radius1=0.3,
        radius2=0,
        depth=0.6
    )
    nose = bpy.context.active_object
    nose.name = f"{name}_Nose"
    nose.rotation_euler = (0, math.radians(-90), 0)
    nose.location = (1.0, 0, 0.5)
    
    # Create wings
    bpy.ops.mesh.primitive_cube_add(size=0.5)
    wings = bpy.context.active_object
    wings.name = f"{name}_Wings"
    wings.scale = (0.8, 2.0, 0.05)
    wings.location = (0, 0, 0.5)
    
    # Create stabilizers (tail fins)
    bpy.ops.mesh.primitive_cube_add(size=0.3)
    horizontal_stabilizer = bpy.context.active_object
    horizontal_stabilizer.name = f"{name}_HorizontalStabilizer"
    horizontal_stabilizer.scale = (0.3, 0.8, 0.05)
    horizontal_stabilizer.location = (-0.8, 0, 0.5)
    
    bpy.ops.mesh.primitive_cube_add(size=0.3)
    vertical_stabilizer = bpy.context.active_object
    vertical_stabilizer.name = f"{name}_VerticalStabilizer"
    vertical_stabilizer.scale = (0.3, 0.05, 0.4)
    vertical_stabilizer.location = (-0.8, 0, 0.7)
    
    # Create cockpit
    bpy.ops.mesh.primitive_uv_sphere_add(
        radius=0.2,
        segments=16,
        ring_count=8
    )
    cockpit = bpy.context.active_object
    cockpit.name = f"{name}_Cockpit"
    cockpit.scale = (0.5, 0.3, 0.3)
    cockpit.location = (0.4, 0, 0.65)
    
    # Create propeller
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.05,
        depth=0.2
    )
    prop_hub = bpy.context.active_object
    prop_hub.name = f"{name}_PropHub"
    prop_hub.rotation_euler = (0, math.radians(90), 0)
    prop_hub.location = (1.2, 0, 0.5)
    
    # Create propeller blades
    for i in range(3):
        angle = i * (2 * math.pi / 3)
        bpy.ops.mesh.primitive_cube_add(size=0.1)
        blade = bpy.context.active_object
        blade.name = f"{name}_Blade_{i}"
        blade.scale = (0.1, 0.6, 0.05)
        blade.rotation_euler = (0, 0, angle)
        blade.location = (1.3, 0, 0.5)
    
    # Create machine guns
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.05,
            depth=0.8
        )
        gun = bpy.context.active_object
        gun.name = f"{name}_Gun_{side}"
        gun.rotation_euler = (0, math.radians(90), 0)
        gun.location = (0.7, side * 0.25, 0.45)
    
    # Create steampunk details (pipes, gauges)
    for i in range(3):
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.03,
            depth=0.4 + random.uniform(0, 0.3)
        )
        pipe = bpy.context.active_object
        pipe.name = f"{name}_Pipe_{i}"
        pipe.rotation_euler = (
            random.uniform(0, math.pi/4),
            random.uniform(0, math.pi/4),
            0
        )
        pipe.location = (
            random.uniform(-0.5, 0.5),
            random.uniform(-0.2, 0.2),
            0.7
        )
    
    # Create a simple material
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.use_nodes = True
    principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
    if principled_bsdf:
        principled_bsdf.inputs[0].default_value = (0.05, 0.15, 0.2, 1.0)  # Dark blue-ish
        principled_bsdf.inputs[4].default_value = 0.6  # Metallic
    
    # Join all parts
    for obj in bpy.context.scene.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    
    if bpy.context.selected_objects:
        bpy.context.view_layer.objects.active = bpy.context.selected_objects[0]
        bpy.ops.object.join()
        
    # Get the joined object
    fighter = bpy.context.active_object
    fighter.name = name
    
    # Apply material
    if fighter.data.materials:
        fighter.data.materials[0] = mat
    else:
        fighter.data.materials.append(mat)
    
    return fighter

# Create air-to-land bomber
def create_bomber():
    """Create a steampunk-style bomber aircraft"""
    name = "Bomber"
    
    # Create main body (fuselage) - larger than fighter
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.4,
        depth=2.5
    )
    fuselage = bpy.context.active_object
    fuselage.name = f"{name}_Fuselage"
    fuselage.rotation_euler = (0, math.radians(90), 0)
    fuselage.location = (0, 0, 0.6)
    
    # Create nose cone
    bpy.ops.mesh.primitive_cone_add(
        radius1=0.4,
        radius2=0,
        depth=0.7
    )
    nose = bpy.context.active_object
    nose.name = f"{name}_Nose"
    nose.rotation_euler = (0, math.radians(-90), 0)
    nose.location = (1.2, 0, 0.6)
    
    # Create wings (larger than fighter)
    bpy.ops.mesh.primitive_cube_add(size=0.5)
    wings = bpy.context.active_object
    wings.name = f"{name}_Wings"
    wings.scale = (1.0, 2.5, 0.05)
    wings.location = (0, 0, 0.6)
    
    # Create stabilizers (tail fins)
    bpy.ops.mesh.primitive_cube_add(size=0.4)
    horizontal_stabilizer = bpy.context.active_object
    horizontal_stabilizer.name = f"{name}_HorizontalStabilizer"
    horizontal_stabilizer.scale = (0.4, 1.0, 0.05)
    horizontal_stabilizer.location = (-1.0, 0, 0.6)
    
    bpy.ops.mesh.primitive_cube_add(size=0.4)
    vertical_stabilizer = bpy.context.active_object
    vertical_stabilizer.name = f"{name}_VerticalStabilizer"
    vertical_stabilizer.scale = (0.4, 0.05, 0.5)
    vertical_stabilizer.location = (-1.0, 0, 0.9)
    
    # Create cockpit
    bpy.ops.mesh.primitive_uv_sphere_add(
        radius=0.25,
        segments=16,
        ring_count=8
    )
    cockpit = bpy.context.active_object
    cockpit.name = f"{name}_Cockpit"
    cockpit.scale = (0.6, 0.4, 0.3)
    cockpit.location = (0.6, 0, 0.8)
    
    # Create engines (two on wings)
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.15,
            depth=0.5
        )
        engine = bpy.context.active_object
        engine.name = f"{name}_Engine_{side}"
        engine.rotation_euler = (0, math.radians(90), 0)
        engine.location = (0.3, side * 1.2, 0.5)
        
        # Create propellers
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.05,
            depth=0.2
        )
        prop_hub = bpy.context.active_object
        prop_hub.name = f"{name}_PropHub_{side}"
        prop_hub.rotation_euler = (0, math.radians(90), 0)
        prop_hub.location = (0.6, side * 1.2, 0.5)
        
        # Create propeller blades
        for i in range(3):
            angle = i * (2 * math.pi / 3)
            bpy.ops.mesh.primitive_cube_add(size=0.1)
            blade = bpy.context.active_object
            blade.name = f"{name}_Blade_{side}_{i}"
            blade.scale = (0.1, 0.5, 0.05)
            blade.rotation_euler = (0, 0, angle)
            blade.location = (0.7, side * 1.2, 0.5)
    
    # Create bomb bay
    bpy.ops.mesh.primitive_cube_add(size=0.5)
    bomb_bay = bpy.context.active_object
    bomb_bay.name = f"{name}_BombBay"
    bomb_bay.scale = (0.7, 0.5, 0.2)
    bomb_bay.location = (0, 0, 0.3)
    
    # Create bombs
    for i in range(2):
        bpy.ops.mesh.primitive_uv_sphere_add(
            radius=0.15
        )
        bomb = bpy.context.active_object
        bomb.name = f"{name}_Bomb_{i}"
        bomb.scale = (0.5, 0.5, 1.0)
        bomb.location = (i * 0.3 - 0.15, 0, 0.2)
    
    # Create steampunk details (pipes, gauges)
    for i in range(5):
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.03,
            depth=0.4 + random.uniform(0, 0.3)
        )
        pipe = bpy.context.active_object
        pipe.name = f"{name}_Pipe_{i}"
        pipe.rotation_euler = (
            random.uniform(0, math.pi/4),
            random.uniform(0, math.pi/4),
            0
        )
        pipe.location = (
            random.uniform(-0.7, 0.7),
            random.uniform(-0.3, 0.3),
            0.8
        )
    
    # Create a simple material
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.use_nodes = True
    principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
    if principled_bsdf:
        principled_bsdf.inputs[0].default_value = (0.1, 0.1, 0.1, 1.0)  # Dark gray
        principled_bsdf.inputs[4].default_value = 0.5  # Metallic
    
    # Join all parts
    for obj in bpy.context.scene.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    
    if bpy.context.selected_objects:
        bpy.context.view_layer.objects.active = bpy.context.selected_objects[0]
        bpy.ops.object.join()
        
    # Get the joined object
    bomber = bpy.context.active_object
    bomber.name = name
    
    # Apply material
    if bomber.data.materials:
        bomber.data.materials[0] = mat
    else:
        bomber.data.materials.append(mat)
    
    return bomber

# Create very large hovering aircraft
def create_large_aircraft():
    """Create a steampunk-style large hovering aircraft"""
    name = "LargeAircraft"
    
    # Create main body (large airship hull)
    bpy.ops.mesh.primitive_uv_sphere_add(
        radius=1.0,
        segments=32,
        ring_count=16
    )
    hull = bpy.context.active_object
    hull.name = f"{name}_Hull"
    hull.scale = (2.0, 1.0, 0.8)
    hull.location = (0, 0, 1.5)
    
    # Create gondola (command cabin) underneath
    bpy.ops.mesh.primitive_cube_add(size=0.8)
    gondola = bpy.context.active_object
    gondola.name = f"{name}_Gondola"
    gondola.scale = (1.2, 0.6, 0.4)
    gondola.location = (0, 0, 0.8)
    
    # Create connecting struts between hull and gondola
    for x_pos in [-0.8, 0, 0.8]:
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.05,
            depth=0.8
        )
        strut = bpy.context.active_object
        strut.name = f"{name}_Strut_{x_pos}"
        strut.location = (x_pos, 0, 1.1)
    
    # Create large propulsion engines
    for side in [-1, 1]:
        for pos in [-0.7, 0.7]:
            bpy.ops.mesh.primitive_cylinder_add(
                radius=0.2,
                depth=0.5
            )
            engine = bpy.context.active_object
            engine.name = f"{name}_Engine_{side}_{pos}"
            engine.rotation_euler = (math.radians(90), 0, 0)
            engine.location = (pos, side * 0.9, 1.5)
            
            # Create propellers
            bpy.ops.mesh.primitive_cylinder_add(
                radius=0.05,
                depth=0.1
            )
            prop_hub = bpy.context.active_object
            prop_hub.name = f"{name}_PropHub_{side}_{pos}"
            prop_hub.rotation_euler = (math.radians(90), 0, 0)
            prop_hub.location = (pos, side * 1.2, 1.5)
            
            # Create propeller blades
            for i in range(4):
                angle = i * (2 * math.pi / 4)
                bpy.ops.mesh.primitive_cube_add(size=0.1)
                blade = bpy.context.active_object
                blade.name = f"{name}_Blade_{side}_{pos}_{i}"
                blade.scale = (0.05, 0.4, 0.1)
                blade.rotation_euler = (angle, 0, 0)
                blade.location = (pos, side * 1.2, 1.5)
    
    # Create fins/stabilizers
    bpy.ops.mesh.primitive_cube_add(size=0.5)
    tail_fin = bpy.context.active_object
    tail_fin.name = f"{name}_TailFin"
    tail_fin.scale = (0.1, 0.5, 0.8)
    tail_fin.location = (-1.5, 0, 1.7)
    
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cube_add(size=0.5)
        side_fin = bpy.context.active_object
        side_fin.name = f"{name}_SideFin_{side}"
        side_fin.scale = (0.7, 0.1, 0.4)
        side_fin.location = (-1.2, side * 0.8, 1.5)
    
    # Create observation deck/bridge
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.3,
        depth=0.3,
        vertices=8
    )
    bridge = bpy.context.active_object
    bridge.name = f"{name}_Bridge"
    bridge.location = (0.8, 0, 1.9)
    
    # Create windows
    for angle in range(0, 360, 45):
        rad_angle = math.radians(angle)
        bpy.ops.mesh.primitive_cube_add(size=0.1)
        window = bpy.context.active_object
        window.name = f"{name}_Window_{angle}"
        window.scale = (0.05, 0.1, 0.1)
        window.location = (
            0.8 + 0.3 * math.cos(rad_angle),
            0.3 * math.sin(rad_angle),
            1.9
        )
    
    # Create steampunk details (pipes, vents, etc.)
    for i in range(8):
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.04,
            depth=0.3 + random.uniform(0, 0.5)
        )
        pipe = bpy.context.active_object
        pipe.name = f"{name}_Pipe_{i}"
        angle = i * (math.pi / 4)
        pipe.location = (
            random.uniform(-1.5, 1.5),
            random.uniform(-0.8, 0.8),
            1.9 + random.uniform(0, 0.3)
        )
        pipe.rotation_euler = (
            random.uniform(0, math.pi/2),
            random.uniform(0, math.pi/2),
            angle
        )
    
    # Create smokestacks
    for i in range(3):
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.08,
            depth=0.4
        )
        stack = bpy.context.active_object
        stack.name = f"{name}_Stack_{i}"
        stack.location = (
            0.5 - i * 0.5,
            0,
            2.1
        )
    
    # Create weapons/turrets
    for side in [-1, 1]:
        for pos in [-0.5, 0.5]:
            bpy.ops.mesh.primitive_cylinder_add(
                radius=0.1,
                depth=0.2
            )
            turret = bpy.context.active_object
            turret.name = f"{name}_Turret_{side}_{pos}"
            turret.location = (pos, side * 0.5, 0.8)
            
            # Create gun barrel
            bpy.ops.mesh.primitive_cylinder_add(
                radius=0.03,
                depth=0.3
            )
            barrel = bpy.context.active_object
            barrel.name = f"{name}_Barrel_{side}_{pos}"
            barrel.rotation_euler = (0, math.radians(90), 0)
            barrel.location = (pos, side * 0.65, 0.8)
    
    # Create a simple material
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.use_nodes = True
    principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
    if principled_bsdf:
        principled_bsdf.inputs[0].default_value = (0.6, 0.5, 0.4, 1.0)  # Brass/copper
        principled_bsdf.inputs[4].default_value = 0.7  # Metallic
        principled_bsdf.inputs[7].default_value = 0.2  # Roughness
    
    # Join all parts
    for obj in bpy.context.scene.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    
    if bpy.context.selected_objects:
        bpy.context.view_layer.objects.active = bpy.context.selected_objects[0]
        bpy.ops.object.join()
        
    # Get the joined object
    aircraft = bpy.context.active_object
    aircraft.name = name
    
    # Apply material
    if aircraft.data.materials:
        aircraft.data.materials[0] = mat
    else:
        aircraft.data.materials.append(mat)
    
    return aircraft

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
    print("\n=== StrategyForge Air Unit Generator ===")
    print("1. Create and render Fighter (air-to-air)")
    print("2. Create and render Bomber (air-to-land)")
    print("3. Create and render Large Hovering Aircraft")
    print("4. Create and render all")
    print("5. Exit")
    
    choice = input("Enter your choice (1-5): ")
    
    if choice == "1":
        fighter = create_fighter()
        render_object_rotations(fighter, output_dir)
    elif choice == "2":
        bomber = create_bomber()
        render_object_rotations(bomber, output_dir)
    elif choice == "3":
        # Adjust camera for large aircraft
        camera = bpy.data.objects['IsometricCamera']
        camera.location = (15, -15, 15)
        
        large_aircraft = create_large_aircraft()
        render_object_rotations(large_aircraft, output_dir)
    elif choice == "4":
        # Create and render all air units
        fighter = create_fighter()
        render_object_rotations(fighter, output_dir)
        
        # Clear scene for next object
        bpy.ops.object.select_all(action='SELECT')
        bpy.ops.object.delete()
        
        bomber = create_bomber()
        render_object_rotations(bomber, output_dir)
        
        # Clear scene for next object
        bpy.ops.object.select_all(action='SELECT')
        bpy.ops.object.delete()
        
        # Adjust camera for large aircraft
        camera = setup_isometric_camera()
        camera.location = (15, -15, 15)
        
        large_aircraft = create_large_aircraft()
        render_object_rotations(large_aircraft, output_dir)
    else:
        print("Exiting...")
    
    print(f"Finished rendering to {output_dir}")

# Run the script
if __name__ == "__main__":
    main()
