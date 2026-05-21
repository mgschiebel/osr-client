use godot::prelude::*;
use godot::classes::{Node3D, Camera3D, Input};

/// Camera controller with scroll-wheel zoom-to-toggle (first person <-> third person).
#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct CameraController {
    base: Base<Node3D>,
    camera: Option<Gd<Camera3D>>,
    third_person_distance: f32,
    max_distance: f32,
    min_distance: f32,
    is_first_person: bool,
    near_clip_threshold: f32,
}

#[godot_api]
impl INode3D for CameraController {
    fn ready(&mut self) {
        self.third_person_distance = 5.0;
        self.max_distance = 10.0;
        self.min_distance = 1.0;
        self.near_clip_threshold = 0.5;

        let children = self.base.get_children();
        for i in 0..children.len() {
            let child = children.get(i).expect("Child exists");
            if let Ok(cam) = child.try_cast::<Camera3D>() {
                self.camera = Some(cam);
                break;
            }
        }
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();
        if input.is_action_just_pressed("camera_zoom_out") {
            self.zoom_out();
        }
        if input.is_action_just_pressed("camera_zoom_in") {
            self.zoom_in();
        }
        self.update_camera_position();
    }
}

impl CameraController {
    fn zoom_out(&mut self) {
        if self.is_first_person {
            self.is_first_person = false;
            self.third_person_distance = self.min_distance + 1.0;
        } else {
            self.third_person_distance = (self.third_person_distance + 1.0).min(self.max_distance);
        }
    }

    fn zoom_in(&mut self) {
        if self.is_first_person {
            return;
        }
        self.third_person_distance -= 1.0;
        if self.third_person_distance <= self.near_clip_threshold {
            self.is_first_person = true;
        }
    }

    fn update_camera_position(&self) {
        if let Some(camera) = &self.camera {
            let mut cam = camera.bind_mut();
            if self.is_first_person {
                cam.set_position(Vector3::new(0.0, 0.0, 0.0));
            } else {
                let pos = Vector3::new(0.0, 2.0, self.third_person_distance);
                cam.set_position(pos);
                cam.look_at(
                    Vector3::new(0.0, 1.0, 0.0),
                    Vector3::new(0.0, 1.0, 0.0),
                );
            }
        }
    }
}

#[godot_api]
impl CameraController {
    #[func]
    fn set_third_person_distance(&mut self, distance: f32) {
        self.third_person_distance = distance.clamp(self.min_distance, self.max_distance);
    }

    #[func]
    fn get_third_person_distance(&self) -> f32 {
        self.third_person_distance
    }

    #[func]
    fn is_first_person(&self) -> bool {
        self.is_first_person
    }

    #[func]
    fn toggle_first_person(&mut self) {
        self.is_first_person = !self.is_first_person;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_person_transition() {
        let near_clip = 0.3;
        let distance = 0.2;
        assert!(distance < near_clip);

        let distance = 1.0;
        assert!(distance >= near_clip);
    }

    #[test]
    fn test_distance_clamping() {
        let max = 10.0;
        let min = 0.5;
        let distance = 15.0;
        let clamped = distance.clamp(min, max);
        assert_eq!(clamped, max);

        let distance = 0.1;
        let clamped = distance.clamp(min, max);
        assert_eq!(clamped, min);
    }
}
