import bpy
import math
import os
import random

# First, let's make sure we have a clean scene
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete()

# Create land-to-land tank
def create_land_tank():
    """Create a steampunk-style land-to-land tank"""
    name = "LandTank"
    
    # Create tank body (main hull)
    bpy.ops.mesh.primitive_cube_add(size=1.5)
    body = bpy.context.active_object
    body.name = f"{name}_Body"
    body.scale = (1.5, 2.0, 0.6)
    body.location = (0, 0, 0.3)
    
    # Create tank turret
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.5,
        depth=0.4
    )
    turret = bpy.context.active_object
    turret.name = f"{name}_Turret"
    turret.location = (0, 0, 0.8)
    
    # Create tank cannon
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.1,
        depth=1.2
    )
    cannon = bpy.context.active_object
    cannon.name = f"{name}_Cannon"
    cannon.location = (0, 0.8, 0.8)
    cannon.rotation_euler = (math.radians(90), 0, 0)
    
    # Create tracks
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cube_add(size=0.5)
        track = bpy.context.active_object
        track.name = f"{name}_Track_{side}"
        track.scale = (1.8, 0.3, 0.2)
        track.location = (side * 0.8, 0, 0.1)
    
    # Add some armor plates and details
    bpy.ops.mesh.primitive_cube_add(size=0.4)
    front_armor = bpy.context.active_object
    front_armor.name = f"{name}_FrontArmor"
    front_armor.scale = (1.4, 0.2, 0.5)
    front_armor.location = (0, 1.1, 0.3)
    front_armor.rotation_euler = (math.radians(30), 0, 0)
    
    # Add some steampunk details (pipes, rivets)
    for i in range(3):
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.05,
            depth=0.8
        )
        pipe = bpy.context.active_object
        pipe.name = f"{name}_Pipe_{i}"
        pipe.rotation_euler = (0, math.radians(90), 0)
        pipe.location = (0.5 - i * 0.5, -0.8, 0.5)
    
    # Create a simple material
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.use_nodes = True
    principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
    if principled_bsdf:
        principled_bsdf.inputs[0].default_value = (0.05, 0.15, 0.05, 1.0)  # Dark green
    
    # Join all parts
    for obj in bpy.context.scene.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    
    if bpy.context.selected_objects:
        bpy.context.view_layer.objects.active = bpy.context.selected_objects[0]
        bpy.ops.object.join()
        
    # Get the joined object
    tank = bpy.context.active_object
    tank.name = name
    
    # Apply material
    if tank.data.materials:
        tank.data.materials[0] = mat
    else:
        tank.data.materials.append(mat)
    
    return tank

# Create land-to-air tank
def create_aa_tank():
    """Create a steampunk-style anti-air tank"""
    name = "AATank"
    
    # Create tank body (main hull)
    bpy.ops.mesh.primitive_cube_add(size=1.5)
    body = bpy.context.active_object
    body.name = f"{name}_Body"
    body.scale = (1.5, 2.0, 0.6)
    body.location = (0, 0, 0.3)
    
    # Create tank turret (larger, more open)
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.6,
        depth=0.3
    )
    turret = bpy.context.active_object
    turret.name = f"{name}_Turret"
    turret.location = (0, 0, 0.8)
    
    # Create anti-air guns (dual barrels pointed upward)
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.08,
            depth=1.0
        )
        barrel = bpy.context.active_object
        barrel.name = f"{name}_Barrel_{side}"
        barrel.location = (side * 0.2, 0, 1.3)
        barrel.rotation_euler = (0, math.radians(30), 0)
    
    # Create mounting platform for guns
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.3,
        depth=0.2
    )
    mount = bpy.context.active_object
    mount.name = f"{name}_Mount"
    mount.location = (0, 0, 1.0)
    
    # Create tracks
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cube_add(size=0.5)
        track = bpy.context.active_object
        track.name = f"{name}_Track_{side}"
        track.scale = (1.8, 0.3, 0.2)
        track.location = (side * 0.8, 0, 0.1)
    
    # Add some armor plates
    bpy.ops.mesh.primitive_cube_add(size=0.4)
    front_armor = bpy.context.active_object
    front_armor.name = f"{name}_FrontArmor"
    front_armor.scale = (1.4, 0.2, 0.5)
    front_armor.location = (0, 1.1, 0.3)
    front_armor.rotation_euler = (math.radians(30), 0, 0)
    
    # Add radar dish
    bpy.ops.mesh.primitive_uv_sphere_add(
        radius=0.3,
        segments=8,
        ring_count=8
    )
    radar = bpy.context.active_object
    radar.name = f"{name}_Radar"
    radar.scale = (1.0, 1.0, 0.2)
    radar.location = (0, -0.6, 1.1)
    
    # Add steampunk details
    for i in range(2):
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.05,
            depth=0.3
        )
        pipe = bpy.context.active_object
        pipe.name = f"{name}_Pipe_{i}"
        pipe.location = (0.3 * (i*2-1), -0.4, 0.8)
    
    # Create a simple material
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.use_nodes = True
    principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
    if principled_bsdf:
        principled_bsdf.inputs[0].default_value = (0.15, 0.15, 0.2, 1.0)  # Blue-ish gray
    
    # Join all parts
    for obj in bpy.context.scene.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    
    if bpy.context.selected_objects:
        bpy.context.view_layer.objects.active = bpy.context.selected_objects[0]
        bpy.ops.object.join()
        
    # Get the joined object
    aa_tank = bpy.context.active_object
    aa_tank.name = name
    
    # Apply material
    if aa_tank.data.materials:
        aa_tank.data.materials[0] = mat
    else:
        aa_tank.data.materials.append(mat)
    
    return aa_tank

# Create artillery unit
def create_artillery():
    """Create a steampunk-style artillery unit"""
    name = "Artillery"
    
    # Create base platform
    bpy.ops.mesh.primitive_cube_add(size=1.2)
    base = bpy.context.active_object
    base.name = f"{name}_Base"
    base.scale = (1.0, 1.3, 0.4)
    base.location = (0, 0, 0.25)
    
    # Create main gun housing
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.3,
        depth=0.6
    )
    housing = bpy.context.active_object
    housing.name = f"{name}_Housing"
    housing.rotation_euler = (math.radians(90), 0, 0)
    housing.location = (0, 0, 0.7)
    
    # Create main gun barrel (long)
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.15,
        depth=2.0
    )
    barrel = bpy.context.active_object
    barrel.name = f"{name}_Barrel"
    barrel.rotation_euler = (0, math.radians(30), 0)
    barrel.location = (0, 0, 0.9)
    
    # Create tracks
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cube_add(size=0.5)
        track = bpy.context.active_object
        track.name = f"{name}_Track_{side}"
        track.scale = (1.3, 0.3, 0.2)
        track.location = (side * 0.6, 0, 0.1)
    
    # Create support legs/stabilizers
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.08,
            depth=1.0
        )
        leg = bpy.context.active_object
        leg.name = f"{name}_Leg_{side}"
        leg.rotation_euler = (0, 0, math.radians(45 * side))
        leg.location = (side * 0.5, -0.5, 0.3)
        
        # Create foot pad
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.15,
            depth=0.05
        )
        foot = bpy.context.active_object
        foot.name = f"{name}_Foot_{side}"
        foot.location = (side * 0.9, -0.9, 0.05)
    
    # Create ammunition storage
    bpy.ops.mesh.primitive_cube_add(size=0.6)
    ammo = bpy.context.active_object
    ammo.name = f"{name}_AmmoStorage"
    ammo.scale = (0.8, 0.5, 0.5)
    ammo.location = (0, -0.8, 0.5)
    
    # Create recoil mechanism
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.1,
        depth=0.5
    )
    recoil = bpy.context.active_object
    recoil.name = f"{name}_Recoil"
    recoil.rotation_euler = (0, math.radians(30), 0)
    recoil.location = (0, 0, 0.6)
    
    # Add steampunk details
    for i in range(4):
        angle = i * (math.pi / 2)
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.03,
            depth=0.2 + random.uniform(0, 0.2)
        )
        pipe = bpy.context.active_object
        pipe.name = f"{name}_Pipe_{i}"
        pipe.location = (
            0.4 * math.cos(angle),
            0.4 * math.sin(angle),
            0.7
        )
    
    # Create a simple material
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.use_nodes = True
    principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
    if principled_bsdf:
        principled_bsdf.inputs[0].default_value = (0.3, 0.25, 0.2, 1.0)  # Brown-ish
        principled_bsdf.inputs[4].default_value = 0.4  # Metallic
    
    # Join all parts
    for obj in bpy.context.scene.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    
    if bpy.context.selected_objects:
        bpy.context.view_layer.objects.active = bpy.context.selected_objects[0]
        bpy.ops.object.join()
        
    # Get the joined object
    artillery = bpy.context.active_object
    artillery.name = name
    
    # Apply material
    if artillery.data.materials:
        artillery.data.materials[0] = mat
    else:
        artillery.data.materials.append(mat)
    
    return artillery

# Create very large tank
def create_large_tank():
    """Create a steampunk-style very large tank"""
    name = "LargeTank"
    
    # Scale factor for "very large" tank
    scale_factor = 2.0
    
    # Create tank body (main hull)
    bpy.ops.mesh.primitive_cube_add(size=1.5 * scale_factor)
    body = bpy.context.active_object
    body.name = f"{name}_Body"
    body.scale = (1.0, 1.3, 0.4)
    body.location = (0, 0, 0.3 * scale_factor)
    
    # Create tank turret
    bpy.ops.mesh.primitive_cylinder_add(
        radius=0.5 * scale_factor,
        depth=0.4 * scale_factor
    )
    turret = bpy.context.active_object
    turret.name = f"{name}_Turret"
    turret.location = (0, 0, 0.8 * scale_factor)
    
    # Create tank cannons (dual)
    for side in [-0.3, 0.3]:
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.1 * scale_factor,
            depth=1.5 * scale_factor
        )
        cannon = bpy.context.active_object
        cannon.name = f"{name}_Cannon_{side}"
        cannon.location = (side * scale_factor, 0.8 * scale_factor, 0.8 * scale_factor)
        cannon.rotation_euler = (math.radians(90), 0, 0)
    
    # Create tracks (extra long)
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cube_add(size=0.5 * scale_factor)
        track = bpy.context.active_object
        track.name = f"{name}_Track_{side}"
        track.scale = (2.0, 0.3, 0.2)
        track.location = (side * 0.8 * scale_factor, 0, 0.1 * scale_factor)
    
    # Add armor plates
    bpy.ops.mesh.primitive_cube_add(size=0.4 * scale_factor)
    front_armor = bpy.context.active_object
    front_armor.name = f"{name}_FrontArmor"
    front_armor.scale = (1.4, 0.2, 0.5)
    front_armor.location = (0, 1.1 * scale_factor, 0.3 * scale_factor)
    front_armor.rotation_euler = (math.radians(30), 0, 0)
    
    # Add side armor skirts
    for side in [-1, 1]:
        bpy.ops.mesh.primitive_cube_add(size=0.2 * scale_factor)
        skirt = bpy.context.active_object
        skirt.name = f"{name}_Skirt_{side}"
        skirt.scale = (2.0, 1.0, 0.1)
        skirt.location = (0, 0, 0.3 * scale_factor)
        skirt.location.x = side * 1.0 * scale_factor
    
    # Add secondary turrets
    for pos in [-0.7, 0.7]:
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.2 * scale_factor,
            depth=0.2 * scale_factor
        )
        small_turret = bpy.context.active_object
        small_turret.name = f"{name}_SmallTurret_{pos}"
        small_turret.location = (pos * scale_factor, 0.5 * scale_factor, 0.6 * scale_factor)
        
        # Add small gun
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.05 * scale_factor,
            depth=0.5 * scale_factor
        )
        small_gun = bpy.context.active_object
        small_gun.name = f"{name}_SmallGun_{pos}"
        small_gun.rotation_euler = (math.radians(90), 0, 0)
        small_gun.location = (pos * scale_factor, 0.7 * scale_factor, 0.6 * scale_factor)
    
    # Add steampunk details (smokestacks, pipes)
    for i in range(2):
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.1 * scale_factor,
            depth=0.6 * scale_factor
        )
        stack = bpy.context.active_object
        stack.name = f"{name}_Stack_{i}"
        stack.location = (-0.4 * scale_factor + i * 0.8 * scale_factor, -0.5 * scale_factor, 0.7 * scale_factor)
    
    # Add pipes
    for i in range(4):
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.05 * scale_factor,
            depth=0.4 * scale_factor + random.uniform(0, 0.3 * scale_factor)
        )
        pipe = bpy.context.active_object
        pipe.name = f"{name}_Pipe_{i}"
        pipe.rotation_euler = (
            random.uniform(0, math.pi/4),
            random.uniform(0, math.pi/4),
            0
        )
        pipe.location = (
            random.uniform(-0.8, 0.8) * scale_factor,
            random.uniform(-0.8, 0.5) * scale_factor,
            0.6 * scale_factor
        )
    
    # Create a simple material
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.use_nodes = True
    principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
    if principled_bsdf:
        principled_bsdf.inputs[0].default_value = (0.1, 0.1, 0.1, 1.0)  # Dark gray
        principled_bsdf.inputs[4].default_value = 0.6  # Metallic
        principled_bsdf.inputs[7].default_value = 0.4  # Roughness
    
    # Join all parts
    for obj in bpy.context.scene.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    
    if bpy.context.selected_objects:
        bpy.context.view_layer.objects.active = bpy.context.selected_objects[0]
        bpy.ops.object.join()
        
    # Get the joined object
    large_tank = bpy.context.active_object
    large_tank.name = name
    
    # Apply material
    if large_tank.data.materials:
        large_tank.data.materials[0] = mat
    else:
        large_tank.data.materials.append(mat)
    
    return large_tank

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
    print("\n=== StrategyForge Combat Unit Generator ===")
    print("1. Create and render Land-to-Land Tank")
    print("2. Create and render Land-to-Air Tank")
    print("3. Create and render Artillery")
    print("4. Create and render Very Large Tank")
    print("5. Create and render all")
    print("6. Exit")
    
    choice = input("Enter your choice (1-6): ")
    
    if choice == "1":
        tank = create_land_tank()
        render_object_rotations(tank, output_dir)
    elif choice == "2":
        aa_tank = create_aa_tank()
        render_object_rotations(aa_tank, output_dir)
    elif choice == "3":
        artillery = create_artillery()
        render_object_rotations(artillery, output_dir)
    elif choice == "4":
        # Adjust camera for large tank
        camera = bpy.data.objects['IsometricCamera']
        camera.location = (15, -15, 15)
        
        large_tank = create_large_tank()
        render_object_rotations(large_tank, output_dir)
    elif choice == "5":
        # Create and render all combat units
        tank = create_land_tank()
        render_object_rotations(tank, output_dir)
        
        # Clear scene for next object
        bpy.ops.object.select_all(action='SELECT')
        bpy.ops.object.delete()
        
        aa_tank = create_aa_tank()
        render_object_rotations(aa_tank, output_dir)
        
        # Clear scene for next object
        bpy.ops.object.select_all(action='SELECT')
        bpy.ops.object.delete()
        
        artillery = create_artillery()
        render_object_rotations(artillery, output_dir)
        
        # Clear scene for next object
        bpy.ops.object.select_all(action='SELECT')
        bpy.ops.object.delete()
        
        # Adjust camera for large tank
        camera = setup_isometric_camera()
        camera.location = (15, -15, 15)
        
        large_tank = create_large_tank()
        render_object_rotations(large_tank, output_dir)
    else:
        print("Exiting...")
    
    print(f"Finished rendering to {output_dir}")

# Run the script
if __name__ == "__main__":
    main()
