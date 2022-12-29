use itoa::Buffer;

mod parts;

struct Lcg {
    seed: u64,
}

impl Lcg {
    fn random(&mut self) -> u32 {
        /* Linear Congruent Generator, posix/glibc [de]rand48 setting, bits [47..0] are output bits */
        self.seed = u64::wrapping_div(
            u64::wrapping_mul(25214903917, self.seed + 11),
            281474976710656,
        );
        self.seed as u32
    }

    fn binomial(&mut self, p: f64) -> bool {
        /* Sample from Binomial distribution with probability p */
        let sample = f64::from(self.random()) * 1.0 / 4294967295.0;
        sample > p
    }

    fn pick_one(&mut self, s: &[&str]) -> String {
        /* Pick one element from list */
        let n = s.len();
        s[self.random() as usize % n].to_string()
    }

    fn pick_one_float(&mut self, s: &[f64]) -> f64 {
        /* Pick one element from list - float version*/
        let n = s.len();
        s[self.random() as usize % n]
    }

    fn pick_a_or_b(&mut self, p: f64, a: &str, b: &str) -> String {
        /* Pick a or b with probability p of picking a */
        if self.binomial(p) {
            a.to_string()
        } else {
            b.to_string()
        }
    }
}

fn linear_congruential_generator(seed: u64) -> Lcg {
    Lcg { seed }
}

#[derive(Debug, Copy, Clone)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

struct Hsv {
    h: f64,
    s: f64,
    v: f64,
}

pub fn generate_seed(seed_string: &str) -> u64 {
    let mut seed = 0;
    for c in seed_string.as_bytes() {
        seed = u64::rotate_left(seed, 8);
        seed ^= *c as u64;
    }
    seed
}

fn f2rgb(r: f64, g: f64, b: f64) -> Rgb {
    // make rgb from 3 floats
    Rgb {
        r: (r * 1.0 / 255.0) as u8,
        g: (g * 1.0 / 255.0) as u8,
        b: (b * 1.0 / 255.0) as u8,
        a: 255,
    }
}

fn f2hsv(h: f64, s: f64, v: f64) -> Hsv {
    // make hsv from 3 floats
    Hsv { h, s, v }
}

fn to_rgb(s: &str) -> Rgb {
    let r = u8::from_str_radix(&s[1..3], 16).unwrap();
    let g = u8::from_str_radix(&s[3..5], 16).unwrap();
    let b = u8::from_str_radix(&s[5..7], 16).unwrap();
    Rgb { r, g, b, a: 255 }
}

impl Rgb {
    fn to_hsv(self) -> Hsv {
        let r = 255.0 * f64::from(self.r);
        let g = 255.0 * f64::from(self.g);
        let b = 255.0 * f64::from(self.b);
        let min = f64::min(f64::min(r, g), b);
        let v = f64::max(f64::max(r, g), b);
        let c = v - min;
        let mut s = 0.0;
        if v != 0.0 {
            s = c / v;
        }
        let mut h = 0.0;
        if min != v {
            if v == r {
                h = (g - b) / c % 6.0;
            }
            if v == g {
                h = (b - r) / c + 2.0;
            }
            if v == b {
                h = (r - g) / c + 4.0;
            }
            h *= 60.0;
            if h < 0.0 {
                h += 360.0;
            }
        }
        f2hsv(h, s, v)
    }

    fn brighter_than(&self, ref_: &Rgb, delta: f64) -> Rgb {
        let primary = self.to_hsv();
        let secondary = ref_.to_hsv();
        if primary.v >= secondary.v + delta {
            return *self;
        }
        let mut primary = primary;
        primary.v = secondary.v + delta;
        if primary.v > 360.0 {
            primary.v = 360.0;
        }
        primary.to_rgb()
    }

    fn darker_than(&self, ref_: &Rgb, delta: f64) -> Rgb {
        let primary = self.to_hsv();
        let secondary = ref_.to_hsv();
        if primary.v <= secondary.v - delta {
            return *self;
        }
        let mut primary = primary;
        primary.v = secondary.v - delta;
        if primary.v < 0.0 {
            primary.v = 0.0;
        }
        primary.to_rgb()
    }

    fn brighter_or_darker_than(&self, ref_: &Rgb, delta: f64) -> Rgb {
        let primary = self.to_hsv();
        let secondary = ref_.to_hsv();
        if primary.v <= secondary.v {
            return self.darker_than(ref_, delta);
        }
        self.brighter_than(ref_, delta)
    }

    fn with_alpha(&self, alpha: f64) -> Rgb {
        Rgb {
            r: self.r,
            g: self.g,
            b: self.b,
            a: (alpha * 255.0) as u8,
        }
    }

    fn html(&self) -> String {
        if self.a == 255 {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        }
    }
}

impl Hsv {
    fn to_rgb(&self) -> Rgb {
        let h = self.h as i32 / 60;
        let f = self.h / 60.0 - h as f64;
        let p = self.v * (1.0 - self.s);
        let q = self.v * (1.0 - self.s * f);
        let t = self.v * (1.0 - self.s * (1.0 - f));
        match h {
            0 | 6 => f2rgb(self.v, t, p),
            1 => f2rgb(q, self.v, p),
            2 => f2rgb(p, self.v, t),
            3 => f2rgb(p, q, self.v),
            4 => f2rgb(t, p, self.v),
            5 => f2rgb(self.v, p, q),
            _ => f2rgb(0.0, 0.0, 0.0),
        }
    }
}

pub enum Mood {
    Sad,
    Happy,
    Surprised,
}

pub fn male_avatar(seed: u64, mood: Option<Mood>) -> String {
    let mut mood = mood;

    let mut g = linear_congruential_generator(seed);
    let skin_color = to_rgb(&g.pick_one(&[
        "#FFDBAC", "#F5CFA0", "#EAC393", "#E0B687", "#CB9E6E", "#B68655", "#A26D3D", "#8D5524",
    ]));
    let hair_color = to_rgb(&g.pick_one(&[
        "#090806", "#2c222b", "#71635a", "#b7a69e", "#b89778", "#a56b46", "#b55239", "#8d4a43",
        "#91553d", "#533d32", "#3b3024", "#554838", "#4e433f", "#504444", "#6a4e42", "#a7856a",
        "#977961",
    ]))
    .brighter_or_darker_than(&skin_color, 17.0);
    let eyes_color = to_rgb(&g.pick_one(&["#76778b", "#697b94", "#647b90", "#5b7c8b", "#588387"]));
    let eyebrows_color = hair_color
        .darker_than(&skin_color, 7.0)
        .darker_than(&hair_color, 10.0);
    let mustache_color = hair_color
        .darker_than(&skin_color, 7.0)
        .with_alpha(g.pick_one_float(&[1.0, 0.75, 0.5]));
    let mouth_color = to_rgb(&g.pick_one(&["#eec1ad", "#dbac98", "#d29985"]))
        .brighter_or_darker_than(&skin_color, 10.0);
    let glasses_color = to_rgb(&g.pick_one(&[
        "#5f705c", "#43677d", "#5e172d", "#ffb67a", "#a04b5d", "#191919", "#323232", "#4b4b4b",
    ]));
    let clothes_color = to_rgb(&g.pick_one(&[
        "#5bc0de", "#5cb85c", "#428bca", "#03396c", "#005b96", "#6497b1", "#1b85b8", "#5a5255",
        "#559e83", "#ae5a41", "#c3cb71", "#666547", "#ffe28a",
    ]));
    let hat_color = to_rgb(&g.pick_one(&[
        "#18293b", "#2e1e05", "#989789", "#3d6ba7", "#517459", "#a62116",
    ]));

    if mood.is_none() {
        // get random mood
        let mood_str = g.pick_one(&["sad", "happy", "surprised"]);
        mood = Some(match mood_str.as_str() {
            "sad" => Mood::Sad,
            "happy" => Mood::Happy,
            "surprised" => Mood::Surprised,
            _ => Mood::Happy,
        });
    }

    let mood = mood.unwrap();

    let mouth = match mood {
        Mood::Sad => parts::male::mouths::SAD,
        Mood::Happy => parts::male::mouths::HAPPY,
        Mood::Surprised => parts::male::mouths::SURPRISED,
    };

    let _mustache = &g.pick_one(&parts::male::MUSTACHE);
    let _glasses = &g.pick_one(&parts::male::GLASSES);
    let _hair = &g.pick_one(&parts::male::HAIR);
    let _hat = &g.pick_one(&parts::HAT);

    let mut svg = [
        parts::SVG_START,
        parts::male::HEAD,
        &g.pick_one(&parts::EYES),
        &g.pick_one(&parts::EYEBROWS),
        &g.pick_a_or_b(0.5, _mustache, ""),
        mouth,
        &g.pick_a_or_b(0.25, _glasses, ""),
        &g.pick_one(&parts::male::CLOTHES),
        &g.pick_a_or_b(0.95, _hair, ""),
        &g.pick_a_or_b(0.05, _hat, ""),
        parts::SVG_END,
    ]
    .join("");

    svg = svg.replace("${skinColor}", &skin_color.html());
    svg = svg.replace("${hairColor}", &hair_color.html());
    svg = svg.replace("${eyesColor}", &eyes_color.html());
    svg = svg.replace("${eyebrowsColor}", &eyebrows_color.html());
    svg = svg.replace("${mustacheColor}", &mustache_color.html());
    svg = svg.replace(
        "${mustacheColorAlpha}",
        Buffer::new().format(mustache_color.a),
    );
    svg = svg.replace("${mouthColor}", &mouth_color.html());
    svg = svg.replace("${glassesColor}", &glasses_color.html());
    svg = svg.replace("${clothesColor}", &clothes_color.html());
    svg = svg.replace("${hatColor}", &hat_color.html());

    svg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_left() {
        let test_seed = "test-string";
        let seed = generate_seed(test_seed);
        assert_eq!(seed, 8371474226319526676);
    }

    #[test]
    fn male_being_generated() {
        male_avatar(100000, None);
    }

    #[test]
    fn male_being_generated_with_input_seed() {
        let seed = generate_seed("test-male");
        male_avatar(seed, None);
    }

    #[test]
    fn male_being_generated_with_input_seed_and_mood() {
        let seed = generate_seed("test-male");
        male_avatar(seed, Some(Mood::Sad));
    }
}
