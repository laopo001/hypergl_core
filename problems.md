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