use std::collections::HashMap;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcessState {
    Idle,         // 空闲状态
    Starting,     // 启动中
    Running,      // 运行中
    Terminating,  // 终止中
    Terminated,   // 已终止
}

/// 定义进程结构体
struct ManagedProcess {
    name: String,
    path: String,
    priority: u32,
    state: Arc<Mutex<ProcessState>>,
    child: Arc<Mutex<Option<Child>>>,
}

impl ManagedProcess {
    /// 创建新的进程实例，初始状态为Idle
    ///
    ///
    fn new(name: String, path: String, priority: u32) -> Self {
        ManagedProcess {
            name,
            path,
            priority,
            state: Arc::new(Mutex::new(ProcessState::Idle)),
            child: Arc::new(Mutex::new(None)),
        }
    }
    /// 启动进程,从Idle状态转换到Starting，start成功转为Running
    fn start(&mut self) {
        let mut state = self.state.lock().unwrap();
        if *state != ProcessState::Idle {
            println!("进程:[{}] 无法启动：不在Idle状态", self.name);
            return;
        }
        //启动可执行文件
        match Command::new(&self.path)
            .arg("10") //测试用 等待10s
            .spawn() {
            Ok(child) => {
                {
                    let mut child_lock = self.child.lock().unwrap();
                    *child_lock = Some(child);
                }
                let state_clone = self.state.clone();
                let name_clone = self.name.clone();
                let child_clone = self.child.clone();

                //监控子进程
                std::thread::spawn(move || {
                    {
                        let mut state = state_clone.lock().unwrap();
                        *state = ProcessState::Running;
                        println!("进程[{}] 已经进入运行状态", name_clone);
                    }

                    //等待子进程退出
                    let mut child_lock = child_clone.lock().unwrap();
                    if let Some(ref mut Child) = *child_lock {
                        match Child.wait() {
                            Ok(status) => {
                                println!("进程[{}] 已退出，状态:{}", name_clone, status);
                            }
                            Err(e) => {
                                println!("等待进程[{}]退出时发生错误:{}", name_clone, e);
                            }
                        }
                        *child_lock = None; //释放子进程句柄
                    }else{
                        println!("进程 '{}' 的子进程句柄不存在。", name_clone);
                    }
                    {
                        let mut state = state_clone.lock().unwrap();
                        *state = ProcessState::Terminated;
                        println!("进程[{}]已终止", name_clone);
                    }
                });
            }
            Err(e) => {
                println!("启动进程[{}]时发生错误:{}",self.name,e);
                let mut state = self.state.lock().unwrap();
                *state = ProcessState::Terminated;
            }
        }
    }


    /// 终止进程，从 Running 状态转换到 Terminating，然后到 Terminated
    fn terminate(&mut self) {
        let mut state = self.state.lock().unwrap();
        if *state == ProcessState::Running {
            *state = ProcessState::Terminating;
            println!("正在终止进程 '{}'...", self.name);

            let mut child_lock = self.child.lock().unwrap();
            if let Some(ref mut child) = *child_lock {
                if let Err(e) = child.kill() {
                    println!("终止进程 '{}' 时发生错误: {}", self.name, e);
                } else {
                    println!("已向进程 '{}' 发送终止信号。", self.name);
                }
            } else {
                println!("进程 '{}' 没有子进程句柄。", self.name);
            }
        } else {
            println!("进程 '{}' 不在运行状态，无法终止。", self.name);
        }
    }


    /// 获取当前的进程状态
    fn get_state(&self) -> ProcessState {
        let state = self.state.lock().unwrap();
        *state
    }


}



pub struct ExecutionManagement {
    processes: HashMap<String, ManagedProcess>,
}

impl ExecutionManagement {
    pub(crate) fn new() -> Self {
        ExecutionManagement {
            processes: HashMap::new(),
        }
    }
    /// 从清单文件加载进程信息
    pub(crate) fn load_manifest(&mut self, manifest_path: &str) {
        // 这里假设清单文件是一个简单的文本文件，每行格式为：
        // name,path,priority
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(manifest_path).expect("无法打开清单文件");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(entry) = line {
                let parts: Vec<&str> = entry.split(',').collect();
                if parts.len() == 3 {
                    let name = parts[0].to_string();
                    let path = parts[1].to_string();
                    let priority = parts[2].parse::<u32>().unwrap_or(0);

                    let process = ManagedProcess::new(name.clone(), path, priority);
                    self.processes.insert(name, process);
                }
            }
        }
    }

    /// 启动所有进程，按照优先级排序
    pub(crate) fn start_all(&mut self) {
        let mut processes: Vec<&mut ManagedProcess> = self.processes.values_mut().collect();
        processes.sort_by_key(|p| p.priority);

        for process in processes {
            process.start();
        }
    }

    /// 终止所有进程
    fn terminate_all(&mut self) {
        for process in self.processes.values_mut() {
            process.terminate();
        }
    }

    /// 监视所有进程状态
    pub(crate) fn monitor(&self) {
        loop {
            let mut all_terminated = true;

            for process in self.processes.values() {
                let state = process.get_state();
                println!("进程 '{}' 当前状态: {:?}", process.name, state);

                if state != ProcessState::Terminated {
                    all_terminated = false;
                }
            }

            if all_terminated {
                break;
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
