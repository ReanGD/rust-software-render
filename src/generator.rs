#[allow(dead_code)]
pub fn generate_square() -> Result<Mesh, String> {
    let mut mesh = Mesh::new();
    let mut vb: Vec<Vertex> = vec![Vertex::new(); 4];
    vb[0].position = Vector3::new(-0.5_f32,  0.5_f32, 0.0_f32);
    vb[1].position = Vector3::new( 0.5_f32,  0.5_f32, 0.0_f32);
    vb[2].position = Vector3::new( 0.5_f32, -0.5_f32, 0.0_f32);
    vb[3].position = Vector3::new(-0.5_f32, -0.5_f32, 0.0_f32);
    mesh.vertex(vb);
    let mut ib: Vec<u32> = vec![0; 2 * 3];
    ib[0] = 0;
    ib[1] = 1;
    ib[2] = 2;
    ib[3] = 0;
    ib[4] = 2;
    ib[5] = 3;
    try!(mesh.index(ib));

    Ok(mesh)
}

#[allow(dead_code)]
pub fn generate_cube() -> Result<Mesh, String> {
    let mut mesh = Mesh::new();
    let mut vb: Vec<Vertex> = vec![Vertex::new(); 24];
	vb[ 0].position	= Vector3::new(-0.5_f32,-0.5_f32,-0.5_f32);
	vb[ 1].position	= Vector3::new(-0.5_f32, 0.5_f32,-0.5_f32);
	vb[ 2].position	= Vector3::new( 0.5_f32, 0.5_f32,-0.5_f32);
	vb[ 3].position	= Vector3::new( 0.5_f32,-0.5_f32,-0.5_f32);
	vb[ 4].position	= Vector3::new( 0.5_f32,-0.5_f32, 0.5_f32);
	vb[ 5].position	= Vector3::new( 0.5_f32, 0.5_f32, 0.5_f32);
	vb[ 6].position	= Vector3::new(-0.5_f32, 0.5_f32, 0.5_f32);
	vb[ 7].position	= Vector3::new(-0.5_f32,-0.5_f32, 0.5_f32);
	vb[ 8].position	= Vector3::new(-0.5_f32,-0.5_f32, 0.5_f32);
	vb[ 9].position	= Vector3::new(-0.5_f32, 0.5_f32, 0.5_f32);
	vb[10].position	= Vector3::new(-0.5_f32, 0.5_f32,-0.5_f32);
	vb[11].position	= Vector3::new(-0.5_f32,-0.5_f32,-0.5_f32);
	vb[12].position	= Vector3::new( 0.5_f32,-0.5_f32,-0.5_f32);
	vb[13].position	= Vector3::new( 0.5_f32, 0.5_f32,-0.5_f32);
	vb[14].position	= Vector3::new( 0.5_f32, 0.5_f32, 0.5_f32);
	vb[15].position	= Vector3::new( 0.5_f32,-0.5_f32, 0.5_f32);
	vb[16].position	= Vector3::new(-0.5_f32,-0.5_f32, 0.5_f32);
	vb[17].position	= Vector3::new(-0.5_f32,-0.5_f32,-0.5_f32);
	vb[18].position	= Vector3::new( 0.5_f32,-0.5_f32,-0.5_f32);
	vb[19].position	= Vector3::new( 0.5_f32,-0.5_f32, 0.5_f32);
	vb[20].position	= Vector3::new(-0.5_f32, 0.5_f32,-0.5_f32);
	vb[21].position	= Vector3::new(-0.5_f32, 0.5_f32, 0.5_f32);
	vb[22].position	= Vector3::new( 0.5_f32, 0.5_f32, 0.5_f32);
	vb[23].position	= Vector3::new( 0.5_f32, 0.5_f32,-0.5_f32);
    mesh.vertex(vb);
    let mut ib: Vec<u32> = vec![0; 12 * 3];
	for i in 0..6 {
		let sm = (i * 4) as u32;
        let ind = i * 6;
		ib[ind + 0] = sm + 0; ib[ind + 1] = sm + 1; ib[ind + 2] = sm + 2;
		ib[ind + 3] = sm + 0; ib[ind + 4] = sm + 2; ib[ind + 5] = sm + 3;
	}
    try!(mesh.index(ib));

    Ok(mesh)
}
