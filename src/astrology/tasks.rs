use super::planets::Planet;
use std::collections::HashMap;

/// Task type classification based on astrological domains
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskType {
    Network,        // Mercury - Communication
    CpuIntensive,   // Mars - Energy/Action
    Desktop,        // Venus - Harmony/UI
    MemoryHeavy,    // Jupiter - Expansion
    System,         // Saturn - Structure
    Interactive,    // Moon - Emotions/Cycles
    #[allow(dead_code)]  // Never returned by classify(), only used in is_critical() check
    Critical,       // Sun - Life Force (only for PID 1/init)
}

impl TaskType {
    /// Get the ruling planet for this task type
    pub fn ruling_planet(self) -> Planet {
        match self {
            TaskType::Network => Planet::Mercury,
            TaskType::CpuIntensive => Planet::Mars,
            TaskType::Desktop => Planet::Venus,
            TaskType::MemoryHeavy => Planet::Jupiter,
            TaskType::System => Planet::Saturn,
            TaskType::Interactive => Planet::Moon,
            TaskType::Critical => Planet::Sun,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            TaskType::Network => "Network",
            TaskType::CpuIntensive => "CPU-Intensive",
            TaskType::Desktop => "Desktop/UI",
            TaskType::MemoryHeavy => "Memory-Heavy",
            TaskType::System => "System",
            TaskType::Interactive => "Interactive",
            TaskType::Critical => "Critical",
        }
    }
}

/// Task classifier - maps process names to task types
pub struct TaskClassifier {
    patterns: HashMap<String, TaskType>,
}

impl TaskClassifier {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        for pattern in &[
            "ssh", "sshd", "curl", "wget", "transmission", "discord", "slack",
            "teams", "zoom", "thunderbird", "evolution", "networkmanager",
            "dhcpcd", "wpa_supplicant", "nginx", "apache", "httpd", "node",
            "npm", "deno",
        ] {
            patterns.insert((*pattern).to_string(), TaskType::Network);
        }

        for pattern in &[
            "cc1", "rustc", "make", "cargo", "gcc", "clang", "g++", "ld",
            "as", "ffmpeg", "blender", "gimp", "inkscape", "handbrake",
            "x264", "x265", "vpxenc", "tar", "gzip", "bzip2", "xz", "zip",
            "7z", "convert", "montage",
        ] {
            patterns.insert((*pattern).to_string(), TaskType::CpuIntensive);
        }

        for pattern in &[
            "gnome-shell", "kde", "plasma", "kwin", "xorg", "wayland",
            "pulseaudio", "pipewire", "mutter", "compiz", "enlightenment",
            "xfce4", "lxde", "mate-panel", "cinnamon", "budgie", "polybar",
            "waybar", "dunst", "mako", "rofi", "dmenu",
        ] {
            patterns.insert((*pattern).to_string(), TaskType::Desktop);
        }

        for pattern in &[
            "postgres", "postgresql", "mysql", "mariadb", "redis", "memcached",
            "mongodb", "cassandra", "elasticsearch", "java", "electron",
            "idea", "pycharm", "studio", "vscode", "code", "docker",
            "containerd", "qemu", "virtualbox",
        ] {
            patterns.insert((*pattern).to_string(), TaskType::MemoryHeavy);
        }

        for pattern in &[
            "systemd", "init", "kworker", "kswapd", "kthreadd", "ksoftirqd",
            "migration", "rcu", "watchdog", "irqbalance", "systemd-journald",
            "systemd-udevd", "systemd-logind", "dbus-daemon", "accounts-daemon",
            "polkitd", "rtkit-daemon", "udisksd", "upowerd",
        ] {
            patterns.insert((*pattern).to_string(), TaskType::System);
        }

        for pattern in &[
            "bash", "zsh", "fish", "sh", "vim", "nvim", "emacs", "nano",
            "less", "more", "cat", "grep", "awk", "sed", "tmux", "screen",
            "htop", "top", "btop", "glances", "alacritty", "kitty", "konsole",
            "gnome-terminal", "terminator", "yakuake", "st",
        ] {
            patterns.insert((*pattern).to_string(), TaskType::Interactive);
        }

        Self { patterns }
    }

    /// Classify a task based on its command name
    pub fn classify(&self, comm: &str) -> TaskType {
        if comm.contains("firefox") || comm.contains("chrome") || comm.contains("chromium") {
            return TaskType::Network;
        }

        if let Some(&task_type) = self.patterns.get(comm) {
            return task_type;
        }

        for (pattern, &task_type) in &self.patterns {
            if comm.starts_with(pattern) || comm.contains(pattern) {
                return task_type;
            }
        }

        TaskType::Interactive
    }

    /// Check if a task is critical (should always get priority regardless of planets)
    #[must_use]
    pub fn is_critical(pid: i32) -> bool {
        pid == 1
    }
}

impl Default for TaskClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_classification_network() {
        let classifier = TaskClassifier::new();

        assert_eq!(classifier.classify("sshd"), TaskType::Network);
        assert_eq!(classifier.classify("firefox"), TaskType::Network);
        assert_eq!(classifier.classify("curl"), TaskType::Network);
        assert_eq!(classifier.classify("ssh").ruling_planet(), Planet::Mercury);
    }

    #[test]
    fn test_task_classification_cpu() {
        let classifier = TaskClassifier::new();

        assert_eq!(classifier.classify("rustc"), TaskType::CpuIntensive);
        assert_eq!(classifier.classify("gcc"), TaskType::CpuIntensive);
        assert_eq!(classifier.classify("ffmpeg"), TaskType::CpuIntensive);
        assert_eq!(classifier.classify("cargo").ruling_planet(), Planet::Mars);
    }

    #[test]
    fn test_task_classification_desktop() {
        let classifier = TaskClassifier::new();

        assert_eq!(classifier.classify("gnome-shell"), TaskType::Desktop);
        assert_eq!(classifier.classify("pulseaudio"), TaskType::Desktop);
        assert_eq!(classifier.classify("xorg"), TaskType::Desktop);
    }

    #[test]
    fn test_task_classification_memory() {
        let classifier = TaskClassifier::new();

        assert_eq!(classifier.classify("postgres"), TaskType::MemoryHeavy);
        assert_eq!(classifier.classify("java"), TaskType::MemoryHeavy);
        assert_eq!(classifier.classify("electron"), TaskType::MemoryHeavy);
    }

    #[test]
    fn test_task_classification_system() {
        let classifier = TaskClassifier::new();

        assert_eq!(classifier.classify("systemd"), TaskType::System);
        assert_eq!(classifier.classify("kworker/0:0"), TaskType::System);
        assert_eq!(classifier.classify("init"), TaskType::System);
    }

    #[test]
    fn test_task_classification_interactive() {
        let classifier = TaskClassifier::new();

        assert_eq!(classifier.classify("bash"), TaskType::Interactive);
        assert_eq!(classifier.classify("vim"), TaskType::Interactive);
        assert_eq!(classifier.classify("tmux"), TaskType::Interactive);
    }

    #[test]
    fn test_task_classification_default() {
        let classifier = TaskClassifier::new();

        assert_eq!(classifier.classify("unknown_process"), TaskType::Interactive);
        assert_eq!(classifier.classify("my_custom_app"), TaskType::Interactive);
    }

    #[test]
    fn test_partial_matching() {
        let classifier = TaskClassifier::new();

        assert_eq!(classifier.classify("kworker/0:1"), TaskType::System);
        assert_eq!(classifier.classify("kworker/1:0H"), TaskType::System);
        assert_eq!(classifier.classify("systemd-journald"), TaskType::System);
    }

    #[test]
    fn test_critical_pid() {
        assert!(TaskClassifier::is_critical(1));
        assert!(!TaskClassifier::is_critical(1000));
        assert!(!TaskClassifier::is_critical(0));
    }

    #[test]
    fn test_ruling_planets() {
        assert_eq!(TaskType::Network.ruling_planet(), Planet::Mercury);
        assert_eq!(TaskType::CpuIntensive.ruling_planet(), Planet::Mars);
        assert_eq!(TaskType::Desktop.ruling_planet(), Planet::Venus);
        assert_eq!(TaskType::MemoryHeavy.ruling_planet(), Planet::Jupiter);
        assert_eq!(TaskType::System.ruling_planet(), Planet::Saturn);
        assert_eq!(TaskType::Interactive.ruling_planet(), Planet::Moon);
        assert_eq!(TaskType::Critical.ruling_planet(), Planet::Sun);
    }
}
