```
thread 'main' panicked at 'wgpu error: Validation Error

Caused by:
    In a RenderPass
      note: encoder = `Render Encoder`
    In a draw command, indexed:true indirect:false
      note: render pipeline = `Render Pipeline`
    index 9 extends beyond limit 0. Did you bind the correct index buffer?

// 注意索引buffer的格式和wgpu::IndexFormat是否一致，不一致也会报没有绑定index_buffer
self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
```

[src/camera.rs:44] &view = [
    [
        0.70710677,
        -0.4082483,
        0.57735026,
        0.0,
    ],
    [
        0.0,
        0.8164966,
        0.57735026,
        0.0,
    ],
    [
        -0.70710677,
        -0.4082483,
        0.57735026,
        0.0,
    ],
    [
        0.0,
        0.0,
        -3.4641016,
        1.0,
    ],
]
[src/camera.rs:44] &proj = [
    [
        2.4142134,
        0.0,
        0.0,
        0.0,
    ],
    [
        0.0,
        2.4142134,
        0.0,
        0.0,
    ],
    [
        0.0,
        0.0,
        -1.002002,
        -1.0,
    ],
    [
        0.0,
        0.0,
        -0.2002002,
        0.0,
    ],
]




[src/ecs/components/camera.rs:36] &self.proj_martix = [
    [
        2.4142134,
        0.0,
        0.0,
        0.0,
    ],
    [
        0.0,
        2.4142134,
        0.0,
        0.0,
    ],
    [
        0.0,
        0.0,
        -1.002002,
        -1.0,
    ],
    [
        0.0,
        0.0,
        -0.2002002,
        0.0,
    ],
]
[src/ecs/components/camera.rs:36] &self.entity.unwrap().as_mut().get_world_matrix() = [
    [
        0.7071067,
        0.0,
        -0.7071068,
        0.0,
    ],
    [
        -0.40824828,
        0.8164966,
        -0.4082482,
        0.0,
    ],
    [
        0.5773504,
        0.5773502,
        0.57735026,
        0.0,
    ],
    [
        2.0,
        2.0,
        2.0,
        1.0,
    ],
]