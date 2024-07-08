use crate::renderer::TILE;

pub fn procces_map(raw_body: Vec<&str>) -> Vec<(usize, usize)> {
    let mut body = Vec::new();
    for (y, j) in raw_body.iter().enumerate() {
        for (x, c) in j.chars().enumerate() {
            if c == '#' {
                body.push((x * TILE as usize, y * TILE as usize));
            }
        }
    }

    body
}
