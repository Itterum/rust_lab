use raylib::prelude::*;

// Структура для описания параметров орбиты и внешнего вида
struct CelestialBody {
    name: String,
    orbit_radius: f32,
    rotation_speed: f32, // Скорость вращения вокруг центра (рад/сек)
    angle: f32,          // Текущий угол в радианах
    size: f32,
    color: Color,
    // Список спутников (например, Луна у Земли)
    satellites: Vec<CelestialBody>,
}

impl CelestialBody {
    fn new(name: &str, orbit_radius: f32, rotation_speed: f32, size: f32, color: Color) -> Self {
        Self {
            name: name.to_string(),
            orbit_radius,
            rotation_speed,
            angle: 0.0,
            size,
            color,
            satellites: Vec::new(),
        }
    }

    // Метод для обновления угла (движение)
    fn update(&mut self, dt: f32) {
        self.angle += self.rotation_speed * dt;

        // Обновляем и все спутники тоже
        for satellite in self.satellites.iter_mut() {
            satellite.update(dt);
        }
    }

    // Рекурсивный метод отрисовки
    fn draw(&self, d: &mut RaylibDrawHandle, center: Vector2) {
        // Вычисляем позицию текущего тела относительно переданного центра
        let x = center.x + self.orbit_radius * self.angle.cos();
        let y = center.y + self.orbit_radius * self.angle.sin();
        let current_pos = Vector2::new(x, y);

        // Рисуем орбиту (просто круглую линию)
        if self.orbit_radius > 0.0 {
            d.draw_circle_lines(
                center.x as i32,
                center.y as i32,
                self.orbit_radius,
                Color::GRAY,
            );
        }

        // Рисуем само тело
        d.draw_circle_v(current_pos, self.size, self.color);

        // Рисуем название (опционально)
        d.draw_text(
            &self.name,
            (x + self.size) as i32,
            (y + self.size) as i32,
            10,
            Color::WHITE,
        );

        // Рисуем спутники, передавая текущую позицию как новый центр
        for satellite in &self.satellites {
            satellite.draw(d, current_pos);
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Solar System 2D")
        .build();

    // Создаем Солнце (радиус орбиты 0, так как оно в центре)
    let sun = CelestialBody::new("Sun", 0.0, 0.0, 30.0, Color::YELLOW);

    // Создаем Землю
    let mut earth = CelestialBody::new("Earth", 150.0, 1.0, 12.0, Color::BLUE);

    // Добавляем Луну Земле
    let moon = CelestialBody::new("Moon", 30.0, 3.0, 4.0, Color::LIGHTGRAY);
    earth.satellites.push(moon);

    // Создаем Марс
    let mars = CelestialBody::new("Mars", 250.0, 0.7, 10.0, Color::RED);

    // Список планет, которые вращаются вокруг Солнца
    let mut planets = vec![earth, mars];

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        // 1. Логика
        for planet in planets.iter_mut() {
            planet.update(dt);
        }

        // 2. Отрисовка
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let screen_center = Vector2::new(
            d.get_screen_width() as f32 / 2.0,
            d.get_screen_height() as f32 / 2.0,
        );

        // Рисуем солнце
        sun.draw(&mut d, screen_center);

        // Рисуем планеты (они внутри себя нарисуют свои спутники)
        for planet in &planets {
            planet.draw(&mut d, screen_center);
        }
    }
}
