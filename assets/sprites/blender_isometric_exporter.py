import bpy
import math
import os

class IsometricSpriteGenerator:
    """A class to help generate isometric sprites for strategy games"""
    
    def __init__(self, output_path=None):
        """Initialize the sprite generator with default settings"""
        # Set default output path if none provided
        if output_path:
            self.output_path = output_path
        else:
            # Try to use a path relative to this script
            try:
                self.output_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "exported_sprites")
            except:
                # If that fails, use path relative to blend file
                self.output_path = os.path.join(os.path.dirname(bpy.data.filepath), "exported_sprites")
        
        # Create output directory if it doesn't exist
        if not os.path.exists(self.output_path):
            try:
                os.makedirs(self.output_path)
            except PermissionError:
                # Fallback to temp directory if we don't have permission
                import tempfile
                self.output_path = os.path.join(tempfile.gettempdir(), "strategy_forge_sprites")
                if not os.path.exists(self.output_path):
                    os.makedirs(self.output_path)
                print(f"Using temporary directory for sprites: {self.output_path}")
        
        # Default render settings
        self.resolution_x = 512
        self.resolution_y = 512
        self.transparent_background = True
        
        # Scene setup tracking
        self.is_setup_complete = False
    
    def setup_scene(self):
        """Set up the scene with proper isometric camera and lighting"""
        # Clear existing objects (optional, uncomment if needed)
        # bpy.ops.object.select_all(action='SELECT')
        # bpy.ops.object.delete()
        
        # Set render engine to Cycles for better quality
        bpy.context.scene.render.engine = 'CYCLES'
        
        # Set up render settings
        bpy.context.scene.render.resolution_x = self.resolution_x
        bpy.context.scene.render.resolution_y = self.resolution_y
        bpy.context.scene.render.resolution_percentage = 100
        bpy.context.scene.render.film_transparent = self.transparent_background
        
        # Create camera if it doesn't exist
        if 'IsometricCamera' not in bpy.data.objects:
            bpy.ops.object.camera_add()
            camera = bpy.context.active_object
            camera.name = 'IsometricCamera'
        else:
            camera = bpy.data.objects['IsometricCamera']
        
        # Position camera for isometric view
        # True isometric angle is approximately 54.736° from horizontal
        camera.rotation_euler = (math.radians(54.736), 0, math.radians(45))
        
        # Place camera at a good distance
        camera.location = (10, -10, 10)
        
        # Set camera as active
        bpy.context.scene.camera = camera
        
        # Add three point lighting
        self._setup_lighting()
        
        # Mark setup as complete
        self.is_setup_complete = True
        print("Isometric scene setup complete")
        
    def _setup_lighting(self):
        """Create a three-point lighting setup"""
        # Remove existing lights (optional)
        # for obj in bpy.data.objects:
        #     if obj.type == 'LIGHT':
        #         bpy.data.objects.remove(obj)
        
        # Key light (main light)
        if 'KeyLight' not in bpy.data.objects:
            bpy.ops.object.light_add(type='AREA')
            key_light = bpy.context.active_object
            key_light.name = 'KeyLight'
            key_light.location = (5, -2, 8)
            key_light.rotation_euler = (math.radians(60), 0, math.radians(20))
            key_light.data.energy = 800
            
        # Fill light (softer light from opposite side)
        if 'FillLight' not in bpy.data.objects:
            bpy.ops.object.light_add(type='AREA')
            fill_light = bpy.context.active_object
            fill_light.name = 'FillLight'
            fill_light.location = (-5, 2, 5)
            fill_light.rotation_euler = (math.radians(45), 0, math.radians(-20))
            fill_light.data.energy = 400
            
        # Back light (rim light)
        if 'BackLight' not in bpy.data.objects:
            bpy.ops.object.light_add(type='AREA')
            back_light = bpy.context.active_object
            back_light.name = 'BackLight'
            back_light.location = (0, 5, 4)
            back_light.rotation_euler = (math.radians(30), math.radians(-20), 0)
            back_light.data.energy = 600
    
    def render_sprite(self, object_name, filename=None, rotation=0):
        """Render the specified object as a sprite"""
        if not self.is_setup_complete:
            self.setup_scene()
            
        # Make sure the object exists
        if object_name not in bpy.data.objects:
            print(f"Error: Object '{object_name}' not found")
            return False
            
        # Select and make the object active
        obj = bpy.data.objects[object_name]
        bpy.ops.object.select_all(action='DESELECT')
        obj.select_set(True)
        bpy.context.view_layer.objects.active = obj
        
        # Apply rotation if specified (useful for multi-direction sprites)
        original_rotation = obj.rotation_euler.copy()
        if rotation != 0:
            obj.rotation_euler.z += math.radians(rotation)
        
        # Focus camera on object
        self._focus_camera_on_object(obj)
        
        # Set output filename
        if filename is None:
            filename = f"{object_name}"
            if rotation != 0:
                filename += f"_rot{rotation}"
                
        output_file = os.path.join(self.output_path, filename)
        bpy.context.scene.render.filepath = output_file
        
        # Render
        bpy.ops.render.render(write_still=True)
        
        # Reset rotation
        if rotation != 0:
            obj.rotation_euler = original_rotation
            
        print(f"Rendered sprite: {output_file}")
        return True
    
    def render_sprite_rotations(self, object_name, angles=None, prefix=None):
        """Render an object from multiple rotations"""
        if angles is None:
            # Default to 4 angles (90° increments)
            angles = [0, 90, 180, 270]
            
        prefix = prefix or object_name
        
        for angle in angles:
            self.render_sprite(
                object_name, 
                filename=f"{prefix}_rot{angle}.png", 
                rotation=angle
            )
    
    def _focus_camera_on_object(self, obj):
        """Adjust camera to focus on the given object"""
        # Calculate object dimensions and center
        camera = bpy.data.objects['IsometricCamera']
        
        # Get the object's bounds
        bounds = [obj.matrix_world @ Vector(corner) for corner in obj.bound_box]
        
        # Calculate the center and size
        center = sum((Vector(b) for b in bounds), Vector()) / 8
        
        # Update camera target (look-at constraint if it exists)
        constraints = [c for c in camera.constraints if c.type == 'TRACK_TO']
        if constraints:
            for constraint in constraints:
                if constraint.target is None:
                    # Create an empty as target
                    bpy.ops.object.empty_add(location=center)
                    empty = bpy.context.active_object
                    empty.name = f"Target_{obj.name}"
                    constraint.target = empty
                else:
                    constraint.target.location = center
        else:
            # Point camera at object
            direction = camera.location - center
            camera.rotation_euler = direction.to_track_quat('Z', 'Y').to_euler()

# Helper function to create a simple unit model for testing
def create_test_unit(name="TestUnit", unit_type="tank"):
    """Create a simple test unit based on the specified type"""
    bpy.ops.object.select_all(action='DESELECT')
    
    if unit_type == "tank":
        # Create tank body
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
        
        # Join all parts
        for obj in bpy.context.scene.objects:
            if obj.name.startswith(name):
                obj.select_set(True)
        
        bpy.ops.object.join()
        bpy.context.active_object.name = name
        
    elif unit_type == "aircraft":
        # Create aircraft body
        bpy.ops.mesh.primitive_cylinder_add(radius=0.4, depth=2.0)
        body = bpy.context.active_object
        body.name = f"{name}_Body"
        body.rotation_euler = (0, math.radians(90), 0)
        body.location = (0, 0, 0.5)
        
        # Create wings
        bpy.ops.mesh.primitive_cube_add(size=0.5)
        wings = bpy.context.active_object
        wings.name = f"{name}_Wings"
        wings.scale = (3.0, 0.05, 0.8)
        wings.location = (0, 0, 0.5)
        
        # Create tail fin
        bpy.ops.mesh.primitive_cube_add(size=0.5)
        tail = bpy.context.active_object
        tail.name = f"{name}_Tail"
        tail.scale = (0.05, 0.4, 0.5)
        tail.location = (0, -0.8, 0.7)
        
        # Join all parts
        for obj in bpy.context.scene.objects:
            if obj.name.startswith(name):
                obj.select_set(True)
        
        bpy.ops.object.join()
        bpy.context.active_object.name = name
        
    elif unit_type == "resource":
        # Create a simple resource pile
        bpy.ops.mesh.primitive_cylinder_add(radius=0.8, depth=0.5)
        resource = bpy.context.active_object
        resource.name = name
        resource.location = (0, 0, 0.25)
        
        # Add some random bumps to make it look less uniform
        bpy.ops.object.modifier_add(type='DISPLACE')
        resource.modifiers["Displace"].strength = 0.1
        
    return bpy.context.active_object

def main():
    """Main function to run the script"""
    # Create sprite generator
    generator = IsometricSpriteGenerator()
    
    # Set up the scene
    generator.setup_scene()
    
    # Ask user what they want to create
    print("\n=== StrategyForge Sprite Generator ===")
    print("1. Create and render a test tank")
    print("2. Create and render a test aircraft")
    print("3. Create and render a test resource")
    print("4. Render an existing object")
    print("5. Exit")
    
    choice = input("Enter your choice (1-5): ")
    
    if choice == "1":
        tank = create_test_unit("Tank", "tank")
        generator.render_sprite_rotations(tank.name)
    elif choice == "2":
        aircraft = create_test_unit("Aircraft", "aircraft")
        generator.render_sprite_rotations(aircraft.name)
    elif choice == "3":
        resource = create_test_unit("Resource", "resource")
        generator.render_sprite(resource.name)
    elif choice == "4":
        obj_name = input("Enter the name of the object to render: ")
        if obj_name in bpy.data.objects:
            generator.render_sprite_rotations(obj_name)
        else:
            print(f"Object '{obj_name}' not found")
    else:
        print("Exiting...")

if __name__ == "__main__":
    # Import Vector for camera focusing
    from mathutils import Vector
    
    # Run main function
    main()
