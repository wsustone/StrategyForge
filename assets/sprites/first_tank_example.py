import bpy
import math
import os

# First, let's make sure we have a clean scene
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete()

# Create a simple tank model
def create_tank(name="HeavyTank"):
    # Create tank body (main hull)
    bpy.ops.mesh.primitive_cube_add(size=1.5)
    body = bpy.context.active_object
    body.name = f"{name}_Body"
    body.scale = (1.5, 2.0, 0.6)
    body.location = (0, 0, 0.3)
    
    # Create tank turret
    bpy.ops.mesh.primitive_cylinder_add(radius=0.5, depth=0.4)
    turret = bpy.context.active_object
    turret.name = f"{name}_Turret"
    turret.location = (0, 0, 0.8)
    
    # Create tank cannon
    bpy.ops.mesh.primitive_cylinder_add(radius=0.1, depth=1.2)
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
    
    # Add some armor plates to make it more interesting
    bpy.ops.mesh.primitive_cube_add(size=0.4)
    front_armor = bpy.context.active_object
    front_armor.name = f"{name}_FrontArmor"
    front_armor.scale = (1.4, 0.2, 0.5)
    front_armor.location = (0, 1.1, 0.3)
    front_armor.rotation_euler = (math.radians(30), 0, 0)
    
    # Join all parts
    for obj in bpy.context.scene.objects:
        if obj.name.startswith(name):
            obj.select_set(True)
    
    bpy.ops.object.join()
    tank = bpy.context.active_object
    tank.name = name
    
    # Add a simple material
    mat = bpy.data.materials.new(name=f"{name}_Material")
    mat.use_nodes = True
    
    # Set color to military green
    principled_bsdf = mat.node_tree.nodes.get('Principled BSDF')
    if principled_bsdf:
        principled_bsdf.inputs[0].default_value = (0.05, 0.15, 0.05, 1.0)  # RGBA
    
    if tank.data.materials:
        tank.data.materials[0] = mat
    else:
        tank.data.materials.append(mat)
    
    return tank

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

# Render the tank from multiple angles
def render_tank_rotations(tank, output_dir, angles=None):
    if angles is None:
        # Default to 4 angles (90° increments)
        angles = [0, 90, 180, 270]
    
    original_rotation = tank.rotation_euler.copy()
    
    for angle in angles:
        # Rotate the tank
        tank.rotation_euler.z = math.radians(angle)
        
        # Set output path
        bpy.context.scene.render.filepath = os.path.join(output_dir, f"{tank.name}_rot{angle}.png")
        
        # Render
        bpy.ops.render.render(write_still=True)
        
        print(f"Rendered {tank.name} at {angle}° rotation")
    
    # Reset rotation
    tank.rotation_euler = original_rotation

# Main execution
def main():
    # Setup the scene
    output_dir = setup_render()
    setup_isometric_camera()
    setup_lighting()
    
    # Create a tank
    tank = create_tank("HeavyTank")
    
    # Render the tank from multiple angles
    render_tank_rotations(tank, output_dir)
    
    print(f"Finished rendering tank sprites to {output_dir}")

# Run the script
if __name__ == "__main__":
    main()
