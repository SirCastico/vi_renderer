use std::sync::{Condvar, Mutex};


struct Buffer{
    buf: Vec<u32>,
}

impl Buffer{
    pub fn new(width: u32, height: u32) -> Self{
        Self { 
            buf: std::iter::repeat(0).take((width*height) as usize).collect(),
        }
    }
}

struct DoubleBufferSwapChainBufferData{
    pub buf_ready: [bool; 2],
    pub front: u8,
}

pub struct DoubleBufferSwapChain{
    buffers: [Mutex<Buffer>; 2],
    cond_vars: [Condvar; 2],
    buf_data: Mutex<DoubleBufferSwapChainBufferData> // see if there is a better way than this
}

// later see if we can enforce reader writer api

impl DoubleBufferSwapChain{
    pub fn new(width: u32, height: u32) -> Self{
        Self {
            buffers: [
                Mutex::new(Buffer::new(width, height)),
                Mutex::new(Buffer::new(width, height)),
            ],
            cond_vars: [
                Condvar::new(),
                Condvar::new(),
            ],
            buf_data: Mutex::new(DoubleBufferSwapChainBufferData{
                buf_ready: [false,false],
                front: 0,
            }),
        }
    }

    pub fn update_back<OP>(&self, f: OP)
        where OP: FnOnce(&mut [u32])
    {
        let b_ind: usize;
        {
            let bdata = self.buf_data.lock().unwrap();
            if !bdata.buf_ready[bdata.front as usize] {
                b_ind = bdata.front as usize;
            } else {
                b_ind = bdata.front as usize;
            }
        }
        {
            let mut b = self.buffers[b_ind].lock().unwrap();
            f(&mut b.buf);
        }

        {
            let mut bdata = self.buf_data.lock().unwrap();
            bdata.buf_ready[b_ind] = true;
        }

        self.cond_vars[b_ind].notify_one();
    }

    pub fn wait_use_front<OP>(&self, f: OP)
        where OP: FnOnce(&[u32])
    {
        let b_mtx;
        {
            let mut bdata = self.buf_data.lock().unwrap();
            while !bdata.buf_ready[bdata.front as usize]{
                bdata = self.cond_vars[bdata.front as usize].wait(bdata).unwrap();
            }
            b_mtx = &self.buffers[bdata.front as usize];
        }
        {
            let b = b_mtx.lock().unwrap();
            f(&b.buf);
        }
        {
            let mut bdata = self.buf_data.lock().unwrap();
            let fr = bdata.front;
            bdata.buf_ready[fr as usize] = false;
            bdata.front ^= 1;
        }
    }

}