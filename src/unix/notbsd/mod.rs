use dox::mem;

pub type sa_family_t = u16;
pub type pthread_key_t = ::c_uint;
pub type speed_t = ::c_uint;
pub type tcflag_t = ::c_uint;
pub type loff_t = ::c_longlong;
pub type clockid_t = ::c_int;
pub type key_t = ::c_int;
pub type id_t = ::c_uint;

pub enum timezone {}

s! {
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [u8; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [::c_char; 108]
    }

    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        __ss_align: ::size_t,
        #[cfg(target_pointer_width = "32")]
        __ss_pad2: [u8; 128 - 2 * 4],
        #[cfg(target_pointer_width = "64")]
        __ss_pad2: [u8; 128 - 2 * 8],
    }

    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: socklen_t,

        #[cfg(any(target_os = "linux",
                  target_os = "emscripten",
                  target_os = "fuchsia"))]
        pub ai_addr: *mut ::sockaddr,

        pub ai_canonname: *mut c_char,

        #[cfg(target_os = "android")]
        pub ai_addr: *mut ::sockaddr,

        pub ai_next: *mut addrinfo,
    }

    pub struct sockaddr_nl {
        pub nl_family: ::sa_family_t,
        nl_pad: ::c_ushort,
        pub nl_pid: u32,
        pub nl_groups: u32
    }

    pub struct sockaddr_ll {
        pub sll_family: ::c_ushort,
        pub sll_protocol: ::c_ushort,
        pub sll_ifindex: ::c_int,
        pub sll_hatype: ::c_ushort,
        pub sll_pkttype: ::c_uchar,
        pub sll_halen: ::c_uchar,
        pub sll_addr: [::c_uchar; 8]
    }

    pub struct fd_set {
        fds_bits: [::c_ulong; FD_SETSIZE / ULONG_SIZE],
    }

    pub struct tm {
        pub tm_sec: ::c_int,
        pub tm_min: ::c_int,
        pub tm_hour: ::c_int,
        pub tm_mday: ::c_int,
        pub tm_mon: ::c_int,
        pub tm_year: ::c_int,
        pub tm_wday: ::c_int,
        pub tm_yday: ::c_int,
        pub tm_isdst: ::c_int,
        pub tm_gmtoff: ::c_long,
        pub tm_zone: *const ::c_char,
    }

    pub struct sched_param {
        pub sched_priority: ::c_int,
        #[cfg(any(target_env = "musl"))]
        pub sched_ss_low_priority: ::c_int,
        #[cfg(any(target_env = "musl"))]
        pub sched_ss_repl_period: ::timespec,
        #[cfg(any(target_env = "musl"))]
        pub sched_ss_init_budget: ::timespec,
        #[cfg(any(target_env = "musl"))]
        pub sched_ss_max_repl: ::c_int,
    }

    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void,
    }

    #[cfg_attr(any(all(target_arch = "x86",
                       not(target_env = "musl"),
                       not(target_os = "android")),
                   target_arch = "x86_64"),
               repr(packed))]
    pub struct epoll_event {
        pub events: ::uint32_t,
        pub u64: ::uint64_t,
    }

    pub struct utsname {
        pub sysname: [::c_char; 65],
        pub nodename: [::c_char; 65],
        pub release: [::c_char; 65],
        pub version: [::c_char; 65],
        pub machine: [::c_char; 65],
        pub domainname: [::c_char; 65]
    }

    pub struct lconv {
        pub decimal_point: *mut ::c_char,
        pub thousands_sep: *mut ::c_char,
        pub grouping: *mut ::c_char,
        pub int_curr_symbol: *mut ::c_char,
        pub currency_symbol: *mut ::c_char,
        pub mon_decimal_point: *mut ::c_char,
        pub mon_thousands_sep: *mut ::c_char,
        pub mon_grouping: *mut ::c_char,
        pub positive_sign: *mut ::c_char,
        pub negative_sign: *mut ::c_char,
        pub int_frac_digits: ::c_char,
        pub frac_digits: ::c_char,
        pub p_cs_precedes: ::c_char,
        pub p_sep_by_space: ::c_char,
        pub n_cs_precedes: ::c_char,
        pub n_sep_by_space: ::c_char,
        pub p_sign_posn: ::c_char,
        pub n_sign_posn: ::c_char,
        pub int_p_cs_precedes: ::c_char,
        pub int_p_sep_by_space: ::c_char,
        pub int_n_cs_precedes: ::c_char,
        pub int_n_sep_by_space: ::c_char,
        pub int_p_sign_posn: ::c_char,
        pub int_n_sign_posn: ::c_char,
    }

    pub struct sigevent {
        pub sigev_value: ::sigval,
        pub sigev_signo: ::c_int,
        pub sigev_notify: ::c_int,
        // Actually a union.  We only expose sigev_notify_thread_id because it's
        // the most useful member
        pub sigev_notify_thread_id: ::c_int,
        #[cfg(target_pointer_width = "64")]
        __unused1: [::c_int; 11],
        #[cfg(target_pointer_width = "32")]
        __unused1: [::c_int; 12]
    }
}

// intentionally not public, only used for fd_set
cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        const ULONG_SIZE: usize = 32;
    } else if #[cfg(target_pointer_width = "64")] {
        const ULONG_SIZE: usize = 64;
    } else {
        // Unknown target_pointer_width
    }
}

pub const EXIT_FAILURE: ::c_int = 1;
pub const EXIT_SUCCESS: ::c_int = 0;
pub const RAND_MAX: ::c_int = 2147483647;
pub const EOF: ::c_int = -1;
pub const SEEK_SET: ::c_int = 0;
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;
pub const _IOFBF: ::c_int = 0;
pub const _IONBF: ::c_int = 2;
pub const _IOLBF: ::c_int = 1;

pub const F_DUPFD: ::c_int = 0;
pub const F_GETFD: ::c_int = 1;
pub const F_SETFD: ::c_int = 2;
pub const F_GETFL: ::c_int = 3;
pub const F_SETFL: ::c_int = 4;

// Linux-specific fcntls
pub const F_SETLEASE: ::c_int = 1024;
pub const F_GETLEASE: ::c_int = 1025;
pub const F_NOTIFY: ::c_int = 1026;
pub const F_DUPFD_CLOEXEC: ::c_int = 1030;
pub const F_SETPIPE_SZ: ::c_int = 1031;
pub const F_GETPIPE_SZ: ::c_int = 1032;

// TODO(#235): Include file sealing fcntls once we have a way to verify them.

pub const SIGTRAP: ::c_int = 5;

pub const PTHREAD_CREATE_JOINABLE: ::c_int = 0;
pub const PTHREAD_CREATE_DETACHED: ::c_int = 1;

pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: clockid_t = 3;
pub const CLOCK_MONOTONIC_RAW: clockid_t = 4;
pub const CLOCK_REALTIME_COARSE: clockid_t = 5;
pub const CLOCK_MONOTONIC_COARSE: clockid_t = 6;
pub const CLOCK_BOOTTIME: clockid_t = 7;
pub const CLOCK_REALTIME_ALARM: clockid_t = 8;
pub const CLOCK_BOOTTIME_ALARM: clockid_t = 9;
// TODO(#247) Someday our Travis shall have glibc 2.21 (released in Sep
// 2014.) See also musl/mod.rs
// pub const CLOCK_SGI_CYCLE: clockid_t = 10;
// pub const CLOCK_TAI: clockid_t = 11;
pub const TIMER_ABSTIME: ::c_int = 1;

pub const RLIMIT_CPU: ::c_int = 0;
pub const RLIMIT_FSIZE: ::c_int = 1;
pub const RLIMIT_DATA: ::c_int = 2;
pub const RLIMIT_STACK: ::c_int = 3;
pub const RLIMIT_CORE: ::c_int = 4;
pub const RLIMIT_LOCKS: ::c_int = 10;
pub const RLIMIT_SIGPENDING: ::c_int = 11;
pub const RLIMIT_MSGQUEUE: ::c_int = 12;
pub const RLIMIT_NICE: ::c_int = 13;
pub const RLIMIT_RTPRIO: ::c_int = 14;

pub const RUSAGE_SELF: ::c_int = 0;

pub const O_RDONLY: ::c_int = 0;
pub const O_WRONLY: ::c_int = 1;
pub const O_RDWR: ::c_int = 2;
pub const O_TMPFILE: ::c_int = 0o20000000 | O_DIRECTORY;

pub const SOCK_CLOEXEC: ::c_int = O_CLOEXEC;

pub const S_IFIFO: ::mode_t = 4096;
pub const S_IFCHR: ::mode_t = 8192;
pub const S_IFBLK: ::mode_t = 24576;
pub const S_IFDIR: ::mode_t = 16384;
pub const S_IFREG: ::mode_t = 32768;
pub const S_IFLNK: ::mode_t = 40960;
pub const S_IFSOCK: ::mode_t = 49152;
pub const S_IFMT: ::mode_t = 61440;
pub const S_IRWXU: ::mode_t = 448;
pub const S_IXUSR: ::mode_t = 64;
pub const S_IWUSR: ::mode_t = 128;
pub const S_IRUSR: ::mode_t = 256;
pub const S_IRWXG: ::mode_t = 56;
pub const S_IXGRP: ::mode_t = 8;
pub const S_IWGRP: ::mode_t = 16;
pub const S_IRGRP: ::mode_t = 32;
pub const S_IRWXO: ::mode_t = 7;
pub const S_IXOTH: ::mode_t = 1;
pub const S_IWOTH: ::mode_t = 2;
pub const S_IROTH: ::mode_t = 4;
pub const F_OK: ::c_int = 0;
pub const R_OK: ::c_int = 4;
pub const W_OK: ::c_int = 2;
pub const X_OK: ::c_int = 1;
pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;
pub const SIGHUP: ::c_int = 1;
pub const SIGINT: ::c_int = 2;
pub const SIGQUIT: ::c_int = 3;
pub const SIGILL: ::c_int = 4;
pub const SIGABRT: ::c_int = 6;
pub const SIGFPE: ::c_int = 8;
pub const SIGKILL: ::c_int = 9;
pub const SIGSEGV: ::c_int = 11;
pub const SIGPIPE: ::c_int = 13;
pub const SIGALRM: ::c_int = 14;
pub const SIGTERM: ::c_int = 15;

pub const PROT_NONE: ::c_int = 0;
pub const PROT_READ: ::c_int = 1;
pub const PROT_WRITE: ::c_int = 2;
pub const PROT_EXEC: ::c_int = 4;

pub const LC_CTYPE: ::c_int = 0;
pub const LC_NUMERIC: ::c_int = 1;
pub const LC_TIME: ::c_int = 2;
pub const LC_COLLATE: ::c_int = 3;
pub const LC_MONETARY: ::c_int = 4;
pub const LC_MESSAGES: ::c_int = 5;
pub const LC_ALL: ::c_int = 6;
pub const LC_CTYPE_MASK: ::c_int = (1 << LC_CTYPE);
pub const LC_NUMERIC_MASK: ::c_int = (1 << LC_NUMERIC);
pub const LC_TIME_MASK: ::c_int = (1 << LC_TIME);
pub const LC_COLLATE_MASK: ::c_int = (1 << LC_COLLATE);
pub const LC_MONETARY_MASK: ::c_int = (1 << LC_MONETARY);
pub const LC_MESSAGES_MASK: ::c_int = (1 << LC_MESSAGES);
// LC_ALL_MASK defined per platform

pub const MAP_FILE: ::c_int = 0x0000;
pub const MAP_SHARED: ::c_int = 0x0001;
pub const MAP_PRIVATE: ::c_int = 0x0002;
pub const MAP_FIXED: ::c_int = 0x0010;

pub const MAP_FAILED: *mut ::c_void = !0 as *mut ::c_void;

// MS_ flags for msync(2)
pub const MS_ASYNC: ::c_int = 0x0001;
pub const MS_INVALIDATE: ::c_int = 0x0002;
pub const MS_SYNC: ::c_int = 0x0004;

// MS_ flags for mount(2)
pub const MS_RDONLY: ::c_ulong = 0x01;
pub const MS_NOSUID: ::c_ulong = 0x02;
pub const MS_NODEV: ::c_ulong = 0x04;
pub const MS_NOEXEC: ::c_ulong = 0x08;
pub const MS_SYNCHRONOUS: ::c_ulong = 0x10;
pub const MS_REMOUNT: ::c_ulong = 0x20;
pub const MS_MANDLOCK: ::c_ulong = 0x40;
pub const MS_DIRSYNC: ::c_ulong = 0x80;
pub const MS_NOATIME: ::c_ulong = 0x0400;
pub const MS_NODIRATIME: ::c_ulong = 0x0800;
pub const MS_BIND: ::c_ulong = 0x1000;
pub const MS_MOVE: ::c_ulong = 0x2000;
pub const MS_REC: ::c_ulong = 0x4000;
pub const MS_SILENT: ::c_ulong = 0x8000;
pub const MS_POSIXACL: ::c_ulong = 0x010000;
pub const MS_UNBINDABLE: ::c_ulong = 0x020000;
pub const MS_PRIVATE: ::c_ulong = 0x040000;
pub const MS_SLAVE: ::c_ulong = 0x080000;
pub const MS_SHARED: ::c_ulong = 0x100000;
pub const MS_RELATIME: ::c_ulong = 0x200000;
pub const MS_KERNMOUNT: ::c_ulong = 0x400000;
pub const MS_I_VERSION: ::c_ulong = 0x800000;
pub const MS_STRICTATIME: ::c_ulong = 0x1000000;
pub const MS_ACTIVE: ::c_ulong = 0x40000000;
pub const MS_NOUSER: ::c_ulong = 0x80000000;
pub const MS_MGC_VAL: ::c_ulong = 0xc0ed0000;
pub const MS_MGC_MSK: ::c_ulong = 0xffff0000;
pub const MS_RMT_MASK: ::c_ulong = 0x800051;

pub const EPERM: ::c_int = 1;
pub const ENOENT: ::c_int = 2;
pub const ESRCH: ::c_int = 3;
pub const EINTR: ::c_int = 4;
pub const EIO: ::c_int = 5;
pub const ENXIO: ::c_int = 6;
pub const E2BIG: ::c_int = 7;
pub const ENOEXEC: ::c_int = 8;
pub const EBADF: ::c_int = 9;
pub const ECHILD: ::c_int = 10;
pub const EAGAIN: ::c_int = 11;
pub const ENOMEM: ::c_int = 12;
pub const EACCES: ::c_int = 13;
pub const EFAULT: ::c_int = 14;
pub const ENOTBLK: ::c_int = 15;
pub const EBUSY: ::c_int = 16;
pub const EEXIST: ::c_int = 17;
pub const EXDEV: ::c_int = 18;
pub const ENODEV: ::c_int = 19;
pub const ENOTDIR: ::c_int = 20;
pub const EISDIR: ::c_int = 21;
pub const EINVAL: ::c_int = 22;
pub const ENFILE: ::c_int = 23;
pub const EMFILE: ::c_int = 24;
pub const ENOTTY: ::c_int = 25;
pub const ETXTBSY: ::c_int = 26;
pub const EFBIG: ::c_int = 27;
pub const ENOSPC: ::c_int = 28;
pub const ESPIPE: ::c_int = 29;
pub const EROFS: ::c_int = 30;
pub const EMLINK: ::c_int = 31;
pub const EPIPE: ::c_int = 32;
pub const EDOM: ::c_int = 33;
pub const ERANGE: ::c_int = 34;
pub const EWOULDBLOCK: ::c_int = EAGAIN;

pub const SCM_RIGHTS: ::c_int = 0x01;
pub const SCM_CREDENTIALS: ::c_int = 0x02;

pub const IPPROTO_RAW: ::c_int = 255;

pub const PROT_GROWSDOWN: ::c_int = 0x1000000;
pub const PROT_GROWSUP: ::c_int = 0x2000000;

pub const MAP_TYPE: ::c_int = 0x000f;

pub const MADV_NORMAL: ::c_int = 0;
pub const MADV_RANDOM: ::c_int = 1;
pub const MADV_SEQUENTIAL: ::c_int = 2;
pub const MADV_WILLNEED: ::c_int = 3;
pub const MADV_DONTNEED: ::c_int = 4;
pub const MADV_REMOVE: ::c_int = 9;
pub const MADV_DONTFORK: ::c_int = 10;
pub const MADV_DOFORK: ::c_int = 11;
pub const MADV_MERGEABLE: ::c_int = 12;
pub const MADV_UNMERGEABLE: ::c_int = 13;
pub const MADV_HWPOISON: ::c_int = 100;

pub const IFF_UP: ::c_int = 0x1;
pub const IFF_BROADCAST: ::c_int = 0x2;
pub const IFF_DEBUG: ::c_int = 0x4;
pub const IFF_LOOPBACK: ::c_int = 0x8;
pub const IFF_POINTOPOINT: ::c_int = 0x10;
pub const IFF_NOTRAILERS: ::c_int = 0x20;
pub const IFF_RUNNING: ::c_int = 0x40;
pub const IFF_NOARP: ::c_int = 0x80;
pub const IFF_PROMISC: ::c_int = 0x100;
pub const IFF_ALLMULTI: ::c_int = 0x200;
pub const IFF_MASTER: ::c_int = 0x400;
pub const IFF_SLAVE: ::c_int = 0x800;
pub const IFF_MULTICAST: ::c_int = 0x1000;
pub const IFF_PORTSEL: ::c_int = 0x2000;
pub const IFF_AUTOMEDIA: ::c_int = 0x4000;
pub const IFF_DYNAMIC: ::c_int = 0x8000;

pub const SOL_IP: ::c_int = 0;
pub const SOL_TCP: ::c_int = 6;
pub const SOL_UDP: ::c_int = 17;
pub const SOL_IPV6: ::c_int = 41;
pub const SOL_ICMPV6: ::c_int = 58;
pub const SOL_RAW: ::c_int = 255;
pub const SOL_DECNET: ::c_int = 261;
pub const SOL_X25: ::c_int = 262;
pub const SOL_PACKET: ::c_int = 263;
pub const SOL_ATM: ::c_int = 264;
pub const SOL_AAL: ::c_int = 265;
pub const SOL_IRDA: ::c_int = 266;
pub const SOL_NETBEUI: ::c_int = 267;
pub const SOL_LLC: ::c_int = 268;
pub const SOL_DCCP: ::c_int = 269;
pub const SOL_NETLINK: ::c_int = 270;
pub const SOL_TIPC: ::c_int = 271;

pub const AF_UNSPEC: ::c_int = 0;
pub const AF_UNIX: ::c_int = 1;
pub const AF_LOCAL: ::c_int = 1;
pub const AF_INET: ::c_int = 2;
pub const AF_AX25: ::c_int = 3;
pub const AF_IPX: ::c_int = 4;
pub const AF_APPLETALK: ::c_int = 5;
pub const AF_NETROM: ::c_int = 6;
pub const AF_BRIDGE: ::c_int = 7;
pub const AF_ATMPVC: ::c_int = 8;
pub const AF_X25: ::c_int = 9;
pub const AF_INET6: ::c_int = 10;
pub const AF_ROSE: ::c_int = 11;
pub const AF_DECnet: ::c_int = 12;
pub const AF_NETBEUI: ::c_int = 13;
pub const AF_SECURITY: ::c_int = 14;
pub const AF_KEY: ::c_int = 15;
pub const AF_NETLINK: ::c_int = 16;
pub const AF_ROUTE: ::c_int = AF_NETLINK;
pub const AF_PACKET: ::c_int = 17;
pub const AF_ASH: ::c_int = 18;
pub const AF_ECONET: ::c_int = 19;
pub const AF_ATMSVC: ::c_int = 20;
pub const AF_RDS: ::c_int = 21;
pub const AF_SNA: ::c_int = 22;
pub const AF_IRDA: ::c_int = 23;
pub const AF_PPPOX: ::c_int = 24;
pub const AF_WANPIPE: ::c_int = 25;
pub const AF_LLC: ::c_int = 26;
pub const AF_CAN: ::c_int = 29;
pub const AF_TIPC: ::c_int = 30;
pub const AF_BLUETOOTH: ::c_int = 31;
pub const AF_IUCV: ::c_int = 32;
pub const AF_RXRPC: ::c_int = 33;
pub const AF_ISDN: ::c_int = 34;
pub const AF_PHONET: ::c_int = 35;
pub const AF_IEEE802154: ::c_int = 36;
pub const AF_CAIF: ::c_int = 37;
pub const AF_ALG: ::c_int = 38;

pub const PF_UNSPEC: ::c_int = AF_UNSPEC;
pub const PF_UNIX: ::c_int = AF_UNIX;
pub const PF_LOCAL: ::c_int = AF_LOCAL;
pub const PF_INET: ::c_int = AF_INET;
pub const PF_AX25: ::c_int = AF_AX25;
pub const PF_IPX: ::c_int = AF_IPX;
pub const PF_APPLETALK: ::c_int = AF_APPLETALK;
pub const PF_NETROM: ::c_int = AF_NETROM;
pub const PF_BRIDGE: ::c_int = AF_BRIDGE;
pub const PF_ATMPVC: ::c_int = AF_ATMPVC;
pub const PF_X25: ::c_int = AF_X25;
pub const PF_INET6: ::c_int = AF_INET6;
pub const PF_ROSE: ::c_int = AF_ROSE;
pub const PF_DECnet: ::c_int = AF_DECnet;
pub const PF_NETBEUI: ::c_int = AF_NETBEUI;
pub const PF_SECURITY: ::c_int = AF_SECURITY;
pub const PF_KEY: ::c_int = AF_KEY;
pub const PF_NETLINK: ::c_int = AF_NETLINK;
pub const PF_ROUTE: ::c_int = AF_ROUTE;
pub const PF_PACKET: ::c_int = AF_PACKET;
pub const PF_ASH: ::c_int = AF_ASH;
pub const PF_ECONET: ::c_int = AF_ECONET;
pub const PF_ATMSVC: ::c_int = AF_ATMSVC;
pub const PF_RDS: ::c_int = AF_RDS;
pub const PF_SNA: ::c_int = AF_SNA;
pub const PF_IRDA: ::c_int = AF_IRDA;
pub const PF_PPPOX: ::c_int = AF_PPPOX;
pub const PF_WANPIPE: ::c_int = AF_WANPIPE;
pub const PF_LLC: ::c_int = AF_LLC;
pub const PF_CAN: ::c_int = AF_CAN;
pub const PF_TIPC: ::c_int = AF_TIPC;
pub const PF_BLUETOOTH: ::c_int = AF_BLUETOOTH;
pub const PF_IUCV: ::c_int = AF_IUCV;
pub const PF_RXRPC: ::c_int = AF_RXRPC;
pub const PF_ISDN: ::c_int = AF_ISDN;
pub const PF_PHONET: ::c_int = AF_PHONET;
pub const PF_IEEE802154: ::c_int = AF_IEEE802154;
pub const PF_CAIF: ::c_int = AF_CAIF;
pub const PF_ALG: ::c_int = AF_ALG;

pub const SOMAXCONN: ::c_int = 128;

pub const MSG_OOB: ::c_int = 1;
pub const MSG_PEEK: ::c_int = 2;
pub const MSG_DONTROUTE: ::c_int = 4;
pub const MSG_CTRUNC: ::c_int = 8;
pub const MSG_TRUNC: ::c_int = 0x20;
pub const MSG_DONTWAIT: ::c_int = 0x40;
pub const MSG_EOR: ::c_int = 0x80;
pub const MSG_WAITALL: ::c_int = 0x100;
pub const MSG_FIN: ::c_int = 0x200;
pub const MSG_SYN: ::c_int = 0x400;
pub const MSG_CONFIRM: ::c_int = 0x800;
pub const MSG_RST: ::c_int = 0x1000;
pub const MSG_ERRQUEUE: ::c_int = 0x2000;
pub const MSG_NOSIGNAL: ::c_int = 0x4000;
pub const MSG_MORE: ::c_int = 0x8000;
pub const MSG_WAITFORONE: ::c_int = 0x10000;
pub const MSG_FASTOPEN: ::c_int = 0x20000000;
pub const MSG_CMSG_CLOEXEC: ::c_int = 0x40000000;

pub const SCM_TIMESTAMP: ::c_int = SO_TIMESTAMP;

pub const SOCK_RAW: ::c_int = 3;
pub const SOCK_RDM: ::c_int = 4;
pub const IP_MULTICAST_IF: ::c_int = 32;
pub const IP_MULTICAST_TTL: ::c_int = 33;
pub const IP_MULTICAST_LOOP: ::c_int = 34;
pub const IP_TTL: ::c_int = 2;
pub const IP_HDRINCL: ::c_int = 3;
pub const IP_ADD_MEMBERSHIP: ::c_int = 35;
pub const IP_DROP_MEMBERSHIP: ::c_int = 36;
pub const IP_TRANSPARENT: ::c_int = 19;
pub const IPV6_ADD_MEMBERSHIP: ::c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: ::c_int = 21;

pub const TCP_NODELAY: ::c_int = 1;
pub const TCP_MAXSEG: ::c_int = 2;
pub const TCP_CORK: ::c_int = 3;
pub const TCP_KEEPIDLE: ::c_int = 4;
pub const TCP_KEEPINTVL: ::c_int = 5;
pub const TCP_KEEPCNT: ::c_int = 6;
pub const TCP_SYNCNT: ::c_int = 7;
pub const TCP_LINGER2: ::c_int = 8;
pub const TCP_DEFER_ACCEPT: ::c_int = 9;
pub const TCP_WINDOW_CLAMP: ::c_int = 10;
pub const TCP_INFO: ::c_int = 11;
pub const TCP_QUICKACK: ::c_int = 12;
pub const TCP_CONGESTION: ::c_int = 13;

pub const IPV6_MULTICAST_LOOP: ::c_int = 19;
pub const IPV6_V6ONLY: ::c_int = 26;

pub const SO_DEBUG: ::c_int = 1;
pub const SO_ORIGINAL_DST: ::c_int = 80;

pub const SHUT_RD: ::c_int = 0;
pub const SHUT_WR: ::c_int = 1;
pub const SHUT_RDWR: ::c_int = 2;

pub const LOCK_SH: ::c_int = 1;
pub const LOCK_EX: ::c_int = 2;
pub const LOCK_NB: ::c_int = 4;
pub const LOCK_UN: ::c_int = 8;

pub const SS_ONSTACK: ::c_int = 1;
pub const SS_DISABLE: ::c_int = 2;

pub const PATH_MAX: ::c_int = 4096;

pub const FD_SETSIZE: usize = 1024;

pub const EPOLLIN: ::c_int = 0x1;
pub const EPOLLPRI: ::c_int = 0x2;
pub const EPOLLOUT: ::c_int = 0x4;
pub const EPOLLRDNORM: ::c_int = 0x40;
pub const EPOLLRDBAND: ::c_int = 0x80;
pub const EPOLLWRNORM: ::c_int = 0x100;
pub const EPOLLWRBAND: ::c_int = 0x200;
pub const EPOLLMSG: ::c_int = 0x400;
pub const EPOLLERR: ::c_int = 0x8;
pub const EPOLLHUP: ::c_int = 0x10;
pub const EPOLLET: ::c_int = 0x80000000;

pub const EPOLL_CTL_ADD: ::c_int = 1;
pub const EPOLL_CTL_MOD: ::c_int = 3;
pub const EPOLL_CTL_DEL: ::c_int = 2;

pub const MNT_DETACH: ::c_int = 0x2;
pub const MNT_EXPIRE: ::c_int = 0x4;

pub const Q_GETFMT: ::c_int = 0x800004;
pub const Q_GETINFO: ::c_int = 0x800005;
pub const Q_SETINFO: ::c_int = 0x800006;
pub const QIF_BLIMITS: ::uint32_t = 1;
pub const QIF_SPACE: ::uint32_t = 2;
pub const QIF_ILIMITS: ::uint32_t = 4;
pub const QIF_INODES: ::uint32_t = 8;
pub const QIF_BTIME: ::uint32_t = 16;
pub const QIF_ITIME: ::uint32_t = 32;
pub const QIF_LIMITS: ::uint32_t = 5;
pub const QIF_USAGE: ::uint32_t = 10;
pub const QIF_TIMES: ::uint32_t = 48;
pub const QIF_ALL: ::uint32_t = 63;

pub const MNT_FORCE: ::c_int = 0x1;

pub const Q_SYNC: ::c_int = 0x800001;
pub const Q_QUOTAON: ::c_int = 0x800002;
pub const Q_QUOTAOFF: ::c_int = 0x800003;
pub const Q_GETQUOTA: ::c_int = 0x800007;
pub const Q_SETQUOTA: ::c_int = 0x800008;

pub const TCIOFF: ::c_int = 2;
pub const TCION: ::c_int = 3;
pub const TCOOFF: ::c_int = 0;
pub const TCOON: ::c_int = 1;
pub const TCIFLUSH: ::c_int = 0;
pub const TCOFLUSH: ::c_int = 1;
pub const TCIOFLUSH: ::c_int = 2;
pub const NL0: ::c_int  = 0x00000000;
pub const NL1: ::c_int  = 0x00000100;
pub const TAB0: ::c_int = 0x00000000;
pub const CR0: ::c_int  = 0x00000000;
pub const FF0: ::c_int  = 0x00000000;
pub const BS0: ::c_int  = 0x00000000;
pub const VT0: ::c_int  = 0x00000000;
pub const VERASE: usize = 2;
pub const VKILL: usize = 3;
pub const VINTR: usize = 0;
pub const VQUIT: usize = 1;
pub const VLNEXT: usize = 15;
pub const IGNBRK: ::tcflag_t = 0x00000001;
pub const BRKINT: ::tcflag_t = 0x00000002;
pub const IGNPAR: ::tcflag_t = 0x00000004;
pub const PARMRK: ::tcflag_t = 0x00000008;
pub const INPCK: ::tcflag_t = 0x00000010;
pub const ISTRIP: ::tcflag_t = 0x00000020;
pub const INLCR: ::tcflag_t = 0x00000040;
pub const IGNCR: ::tcflag_t = 0x00000080;
pub const ICRNL: ::tcflag_t = 0x00000100;
pub const IXANY: ::tcflag_t = 0x00000800;
pub const IMAXBEL: ::tcflag_t = 0x00002000;
pub const IUTF8: ::tcflag_t = 0x00004000;
pub const OPOST: ::tcflag_t = 0x1;
pub const CS5: ::tcflag_t = 0x00000000;
pub const CMSPAR: ::tcflag_t = 0o10000000000;
pub const CRTSCTS: ::tcflag_t = 0x80000000;
pub const ECHO: ::tcflag_t = 0x00000008;
pub const OCRNL:  ::tcflag_t = 0o000010;
pub const ONOCR:  ::tcflag_t = 0o000020;
pub const ONLRET: ::tcflag_t = 0o000040;
pub const OFILL:  ::tcflag_t = 0o000100;
pub const OFDEL:  ::tcflag_t = 0o000200;

pub const CLONE_VM: ::c_int = 0x100;
pub const CLONE_FS: ::c_int = 0x200;
pub const CLONE_FILES: ::c_int = 0x400;
pub const CLONE_SIGHAND: ::c_int = 0x800;
pub const CLONE_PTRACE: ::c_int = 0x2000;
pub const CLONE_VFORK: ::c_int = 0x4000;
pub const CLONE_PARENT: ::c_int = 0x8000;
pub const CLONE_THREAD: ::c_int = 0x10000;
pub const CLONE_NEWNS: ::c_int = 0x20000;
pub const CLONE_SYSVSEM: ::c_int = 0x40000;
pub const CLONE_SETTLS: ::c_int = 0x80000;
pub const CLONE_PARENT_SETTID: ::c_int = 0x100000;
pub const CLONE_CHILD_CLEARTID: ::c_int = 0x200000;
pub const CLONE_DETACHED: ::c_int = 0x400000;
pub const CLONE_UNTRACED: ::c_int = 0x800000;
pub const CLONE_CHILD_SETTID: ::c_int = 0x01000000;
pub const CLONE_NEWUTS: ::c_int = 0x04000000;
pub const CLONE_NEWIPC: ::c_int = 0x08000000;
pub const CLONE_NEWUSER: ::c_int = 0x10000000;
pub const CLONE_NEWPID: ::c_int = 0x20000000;
pub const CLONE_NEWNET: ::c_int = 0x40000000;
pub const CLONE_IO: ::c_int = 0x80000000;
pub const CLONE_NEWCGROUP: ::c_int = 0x02000000;

pub const WNOHANG: ::c_int = 0x00000001;
pub const WUNTRACED: ::c_int = 0x00000002;
pub const WSTOPPED: ::c_int = WUNTRACED;
pub const WEXITED: ::c_int = 0x00000004;
pub const WCONTINUED: ::c_int = 0x00000008;
pub const WNOWAIT: ::c_int = 0x01000000;

// Options set using PTRACE_SETOPTIONS.
pub const PTRACE_O_TRACESYSGOOD: ::c_int = 0x00000001;
pub const PTRACE_O_TRACEFORK: ::c_int = 0x00000002;
pub const PTRACE_O_TRACEVFORK: ::c_int = 0x00000004;
pub const PTRACE_O_TRACECLONE: ::c_int = 0x00000008;
pub const PTRACE_O_TRACEEXEC: ::c_int = 0x00000010;
pub const PTRACE_O_TRACEVFORKDONE: ::c_int = 0x00000020;
pub const PTRACE_O_TRACEEXIT: ::c_int = 0x00000040;
pub const PTRACE_O_TRACESECCOMP: ::c_int = 0x00000080;
pub const PTRACE_O_EXITKILL: ::c_int = 0x00100000;
pub const PTRACE_O_SUSPEND_SECCOMP: ::c_int = 0x00200000;
pub const PTRACE_O_MASK: ::c_int = 0x003000ff;

// Wait extended result codes for the above trace options.
pub const PTRACE_EVENT_FORK: ::c_int = 1;
pub const PTRACE_EVENT_VFORK: ::c_int = 2;
pub const PTRACE_EVENT_CLONE: ::c_int = 3;
pub const PTRACE_EVENT_EXEC: ::c_int = 4;
pub const PTRACE_EVENT_VFORK_DONE: ::c_int = 5;
pub const PTRACE_EVENT_EXIT: ::c_int = 6;
pub const PTRACE_EVENT_SECCOMP: ::c_int = 7;
pub const PTRACE_EVENT_STOP: ::c_int = 128;

pub const __WNOTHREAD: ::c_int = 0x20000000;
pub const __WALL: ::c_int = 0x40000000;
pub const __WCLONE: ::c_int = 0x80000000;

pub const SPLICE_F_MOVE: ::c_uint = 0x01;
pub const SPLICE_F_NONBLOCK: ::c_uint = 0x02;
pub const SPLICE_F_MORE: ::c_uint = 0x04;
pub const SPLICE_F_GIFT: ::c_uint = 0x08;

pub const RTLD_LOCAL: ::c_int = 0;

pub const POSIX_FADV_NORMAL: ::c_int = 0;
pub const POSIX_FADV_RANDOM: ::c_int = 1;
pub const POSIX_FADV_SEQUENTIAL: ::c_int = 2;
pub const POSIX_FADV_WILLNEED: ::c_int = 3;

pub const AT_FDCWD: ::c_int = -100;
pub const AT_SYMLINK_NOFOLLOW: ::c_int = 0x100;
pub const AT_REMOVEDIR: ::c_int = 0x200;
pub const AT_SYMLINK_FOLLOW: ::c_int = 0x400;
pub const AT_NO_AUTOMOUNT: ::c_int = 0x800;
pub const AT_EMPTY_PATH: ::c_int = 0x1000;

pub const LOG_CRON: ::c_int = 9 << 3;
pub const LOG_AUTHPRIV: ::c_int = 10 << 3;
pub const LOG_FTP: ::c_int = 11 << 3;
pub const LOG_PERROR: ::c_int = 0x20;

pub const PIPE_BUF: usize = 4096;

pub const SI_LOAD_SHIFT: ::c_uint = 16;

pub const SIGEV_SIGNAL: ::c_int = 0;
pub const SIGEV_NONE: ::c_int = 1;
pub const SIGEV_THREAD: ::c_int = 2;

pub const P_ALL: idtype_t = 0;
pub const P_PID: idtype_t = 1;
pub const P_PGID: idtype_t = 2;

pub const UTIME_OMIT: c_long = 1073741822;
pub const UTIME_NOW: c_long = 1073741823;

pub const POLLRDNORM: ::c_short = 0x040;
pub const POLLRDBAND: ::c_short = 0x080;

f! {
    pub fn FD_CLR(fd: ::c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        let size = mem::size_of_val(&(*set).fds_bits[0]) * 8;
        (*set).fds_bits[fd / size] &= !(1 << (fd % size));
        return
    }

    pub fn FD_ISSET(fd: ::c_int, set: *mut fd_set) -> bool {
        let fd = fd as usize;
        let size = mem::size_of_val(&(*set).fds_bits[0]) * 8;
        return ((*set).fds_bits[fd / size] & (1 << (fd % size))) != 0
    }

    pub fn FD_SET(fd: ::c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        let size = mem::size_of_val(&(*set).fds_bits[0]) * 8;
        (*set).fds_bits[fd / size] |= 1 << (fd % size);
        return
    }

    pub fn FD_ZERO(set: *mut fd_set) -> () {
        for slot in (*set).fds_bits.iter_mut() {
            *slot = 0;
        }
    }

    pub fn WIFSTOPPED(status: ::c_int) -> bool {
        (status & 0xff) == 0x7f
    }

    pub fn WSTOPSIG(status: ::c_int) -> ::c_int {
        (status >> 8) & 0xff
    }

    pub fn WIFCONTINUED(status: ::c_int) -> bool {
        status == 0xffff
    }

    pub fn WIFSIGNALED(status: ::c_int) -> bool {
        ((status & 0x7f) + 1) as i8 >= 2
    }

    pub fn WTERMSIG(status: ::c_int) -> ::c_int {
        status & 0x7f
    }

    pub fn WIFEXITED(status: ::c_int) -> bool {
        (status & 0x7f) == 0
    }

    pub fn WEXITSTATUS(status: ::c_int) -> ::c_int {
        (status >> 8) & 0xff
    }

    pub fn WCOREDUMP(status: ::c_int) -> bool {
        (status & 0x80) != 0
    }
}

extern {
    pub fn getpwnam_r(name: *const ::c_char,
                      pwd: *mut passwd,
                      buf: *mut ::c_char,
                      buflen: ::size_t,
                      result: *mut *mut passwd) -> ::c_int;
    pub fn getpwuid_r(uid: ::uid_t,
                      pwd: *mut passwd,
                      buf: *mut ::c_char,
                      buflen: ::size_t,
                      result: *mut *mut passwd) -> ::c_int;
    pub fn fdatasync(fd: ::c_int) -> ::c_int;
    pub fn mincore(addr: *mut ::c_void, len: ::size_t,
                   vec: *mut ::c_uchar) -> ::c_int;
    pub fn clock_getres(clk_id: clockid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn clock_gettime(clk_id: clockid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn clock_nanosleep(clk_id: clockid_t,
                           flags: ::c_int,
                           rqtp: *const ::timespec,
                           rmtp:  *mut ::timespec) -> ::c_int;
    pub fn clock_settime(clk_id: clockid_t, tp: *const ::timespec) -> ::c_int;
    pub fn dirfd(dirp: *mut ::DIR) -> ::c_int;
    pub fn settimeofday(tv: *const ::timeval, tz: *const ::timezone) -> ::c_int;

    pub fn prctl(option: ::c_int, ...) -> ::c_int;
    pub fn pthread_getattr_np(native: ::pthread_t,
                              attr: *mut ::pthread_attr_t) -> ::c_int;
    pub fn pthread_attr_getguardsize(attr: *const ::pthread_attr_t,
                                     guardsize: *mut ::size_t) -> ::c_int;
    pub fn pthread_attr_getstack(attr: *const ::pthread_attr_t,
                                 stackaddr: *mut *mut ::c_void,
                                 stacksize: *mut ::size_t) -> ::c_int;
    pub fn memalign(align: ::size_t, size: ::size_t) -> *mut ::c_void;
    pub fn setgroups(ngroups: ::size_t,
                     ptr: *const ::gid_t) -> ::c_int;
    pub fn initgroups(user: *const ::c_char, group: ::gid_t) -> ::c_int;
    pub fn sched_setscheduler(pid: ::pid_t,
                              policy: ::c_int,
                              param: *const sched_param) -> ::c_int;
    pub fn sched_getscheduler(pid: ::pid_t) -> ::c_int;
    pub fn sched_get_priority_max(policy: ::c_int) -> ::c_int;
    pub fn sched_get_priority_min(policy: ::c_int) -> ::c_int;
    pub fn sched_setparam(pid: ::pid_t, param: *const sched_param) -> ::c_int;
    pub fn sched_getparam(pid: ::pid_t, param: *mut sched_param) -> ::c_int;
    pub fn sched_rr_get_interval(pid: ::pid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn epoll_create(size: ::c_int) -> ::c_int;
    pub fn epoll_create1(flags: ::c_int) -> ::c_int;
    pub fn epoll_ctl(epfd: ::c_int,
                     op: ::c_int,
                     fd: ::c_int,
                     event: *mut epoll_event) -> ::c_int;
    pub fn epoll_wait(epfd: ::c_int,
                      events: *mut epoll_event,
                      maxevents: ::c_int,
                      timeout: ::c_int) -> ::c_int;
    pub fn pipe2(fds: *mut ::c_int, flags: ::c_int) -> ::c_int;
    pub fn mount(src: *const ::c_char,
                 target: *const ::c_char,
                 fstype: *const ::c_char,
                 flags: ::c_ulong,
                 data: *const ::c_void) -> ::c_int;
    pub fn umount(target: *const ::c_char) -> ::c_int;
    pub fn umount2(target: *const ::c_char, flags: ::c_int) -> ::c_int;
    pub fn clone(cb: extern fn(*mut ::c_void) -> ::c_int,
                 child_stack: *mut ::c_void,
                 flags: ::c_int,
                 arg: *mut ::c_void, ...) -> ::c_int;
    pub fn statfs(path: *const ::c_char, buf: *mut statfs) -> ::c_int;
    pub fn fstatfs(fd: ::c_int, buf: *mut statfs) -> ::c_int;
    pub fn memrchr(cx: *const ::c_void,
                   c: ::c_int,
                   n: ::size_t) -> *mut ::c_void;
    pub fn syscall(num: ::c_long, ...) -> ::c_long;
    pub fn sendfile(out_fd: ::c_int,
                    in_fd: ::c_int,
                    offset: *mut off_t,
                    count: ::size_t) -> ::ssize_t;
    pub fn splice(fd_in: ::c_int,
                  off_in: *mut ::loff_t,
                  fd_out: ::c_int,
                  off_out: *mut ::loff_t,
                  len: ::size_t,
                  flags: ::c_uint) -> ::ssize_t;
    pub fn tee(fd_in: ::c_int,
               fd_out: ::c_int,
               len: ::size_t,
               flags: ::c_uint) -> ::ssize_t;
    pub fn vmsplice(fd: ::c_int,
                    iov: *const ::iovec,
                    nr_segs: ::size_t,
                    flags: ::c_uint) -> ::ssize_t;

    pub fn posix_fadvise(fd: ::c_int, offset: ::off_t, len: ::off_t,
                         advise: ::c_int) -> ::c_int;
    pub fn futimens(fd: ::c_int, times: *const ::timespec) -> ::c_int;
    pub fn utimensat(dirfd: ::c_int, path: *const ::c_char,
                     times: *const ::timespec, flag: ::c_int) -> ::c_int;
    pub fn duplocale(base: ::locale_t) -> ::locale_t;
    pub fn freelocale(loc: ::locale_t);
    pub fn newlocale(mask: ::c_int,
                     locale: *const ::c_char,
                     base: ::locale_t) -> ::locale_t;
    pub fn uselocale(loc: ::locale_t) -> ::locale_t;
    pub fn creat64(path: *const c_char, mode: mode_t) -> ::c_int;
    pub fn fstat64(fildes: ::c_int, buf: *mut stat64) -> ::c_int;
    pub fn ftruncate64(fd: ::c_int, length: off64_t) -> ::c_int;
    pub fn getrlimit64(resource: ::c_int, rlim: *mut rlimit64) -> ::c_int;
    pub fn lseek64(fd: ::c_int, offset: off64_t, whence: ::c_int) -> off64_t;
    pub fn lstat64(path: *const c_char, buf: *mut stat64) -> ::c_int;
    pub fn mmap64(addr: *mut ::c_void,
                  len: ::size_t,
                  prot: ::c_int,
                  flags: ::c_int,
                  fd: ::c_int,
                  offset: off64_t)
                  -> *mut ::c_void;
    pub fn open64(path: *const c_char, oflag: ::c_int, ...) -> ::c_int;
    pub fn openat64(fd: ::c_int,
                    path: *const c_char,
                    oflag: ::c_int, ...) -> ::c_int;
    pub fn pread64(fd: ::c_int, buf: *mut ::c_void, count: ::size_t,
                   offset: off64_t) -> ::ssize_t;
    pub fn pwrite64(fd: ::c_int, buf: *const ::c_void, count: ::size_t,
                    offset: off64_t) -> ::ssize_t;
    pub fn readdir64(dirp: *mut ::DIR) -> *mut ::dirent64;
    pub fn readdir64_r(dirp: *mut ::DIR, entry: *mut ::dirent64,
                       result: *mut *mut ::dirent64) -> ::c_int;
    pub fn setrlimit64(resource: ::c_int, rlim: *const rlimit64) -> ::c_int;
    pub fn stat64(path: *const c_char, buf: *mut stat64) -> ::c_int;
    pub fn truncate64(path: *const c_char, length: off64_t) -> ::c_int;
    pub fn eventfd(init: ::c_uint, flags: ::c_int) -> ::c_int;
    pub fn sysinfo (info: *mut ::sysinfo) -> ::c_int;

    pub fn fdopendir(fd: ::c_int) -> *mut ::DIR;

    pub fn mknodat(dirfd: ::c_int, pathname: *const ::c_char,
                   mode: ::mode_t, dev: dev_t) -> ::c_int;
    pub fn ppoll(fds: *mut ::pollfd,
                 nfds: nfds_t,
                 timeout: *const ::timespec,
                 sigmask: *const sigset_t) -> ::c_int;
    pub fn pthread_condattr_getclock(attr: *const pthread_condattr_t,
                                     clock_id: *mut clockid_t) -> ::c_int;
    pub fn pthread_condattr_setclock(attr: *mut pthread_condattr_t,
                                     clock_id: clockid_t) -> ::c_int;
    pub fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t,
                                       pshared: ::c_int) -> ::c_int;
    pub fn pthread_condattr_getpshared(attr: *const pthread_condattr_t,
                                       pshared: *mut ::c_int) -> ::c_int;
    pub fn pthread_getschedparam(native: ::pthread_t,
                                 policy: *mut ::c_int,
                                 param: *mut ::sched_param) -> ::c_int;
    pub fn pthread_setschedparam(native: ::pthread_t,
                                 policy: ::c_int,
                                 param: *const ::sched_param) -> ::c_int;
    pub fn sched_getaffinity(pid: ::pid_t,
                             cpusetsize: ::size_t,
                             cpuset: *mut cpu_set_t) -> ::c_int;
    pub fn sched_setaffinity(pid: ::pid_t,
                             cpusetsize: ::size_t,
                             cpuset: *const cpu_set_t) -> ::c_int;
    pub fn unshare(flags: ::c_int) -> ::c_int;
    pub fn setns(fd: ::c_int, nstype: ::c_int) -> ::c_int;
    pub fn sem_timedwait(sem: *mut sem_t,
                         abstime: *const ::timespec) -> ::c_int;
    pub fn accept4(fd: ::c_int, addr: *mut ::sockaddr, len: *mut ::socklen_t,
                   flg: ::c_int) -> ::c_int;
    pub fn pthread_mutex_timedlock(lock: *mut pthread_mutex_t,
                                   abstime: *const ::timespec) -> ::c_int;
    pub fn pthread_mutexattr_setpshared(attr: *mut pthread_mutexattr_t,
                                        pshared: ::c_int) -> ::c_int;
    pub fn pthread_mutexattr_getpshared(attr: *const pthread_mutexattr_t,
                                        pshared: *mut ::c_int) -> ::c_int;
    pub fn pthread_rwlockattr_getpshared(attr: *const pthread_rwlockattr_t,
                                         val: *mut ::c_int) -> ::c_int;
    pub fn pthread_rwlockattr_setpshared(attr: *mut pthread_rwlockattr_t,
                                         val: ::c_int) -> ::c_int;
    pub fn ptsname_r(fd: ::c_int,
                     buf: *mut ::c_char,
                     buflen: ::size_t) -> ::c_int;
    pub fn clearenv() -> ::c_int;
    pub fn waitid(idtype: idtype_t, id: id_t, infop: *mut ::siginfo_t,
                  options: ::c_int) -> ::c_int;
    pub fn sigsuspend(mask: *const ::sigset_t) -> ::c_int;
    pub fn setreuid(ruid: ::uid_t, euid: ::uid_t) -> ::c_int;
    pub fn setregid(rgid: ::gid_t, egid: ::gid_t) -> ::c_int;
    pub fn getresuid(ruid: *mut ::uid_t, euid: *mut ::uid_t,
                     suid: *mut ::uid_t) -> ::c_int;
    pub fn getresgid(rgid: *mut ::gid_t, egid: *mut ::gid_t,
                     sgid: *mut ::gid_t) -> ::c_int;
    pub fn personality(persona: ::c_ulong) -> ::c_int;
    pub fn swapon(path: *const ::c_char, swapflags: ::c_int) -> ::c_int;
    pub fn swapoff(puath: *const ::c_char) -> ::c_int;
    pub fn acct(filename: *const ::c_char) -> ::c_int;
    pub fn brk(addr: *mut ::c_void) -> ::c_int;
    pub fn sbrk(increment: ::intptr_t) -> *mut ::c_void;
    pub fn vfork() -> ::pid_t;
    pub fn sethostname(name: *const ::c_char, len: ::size_t) -> ::c_int;
    pub fn setresgid(rgid: ::gid_t, egid: ::gid_t, sgid: ::gid_t) -> ::c_int;
    pub fn setresuid(ruid: ::uid_t, euid: ::uid_t, suid: ::uid_t) -> ::c_int;
    pub fn wait4(pid: ::pid_t, status: *mut ::c_int, options: ::c_int,
                 rusage: *mut ::rusage) -> ::pid_t;
    pub fn openpty(amaster: *mut ::c_int,
                aslave: *mut ::c_int,
                name: *mut ::c_char,
                termp: *const termios,
                winp: *const ::winsize) -> ::c_int;
    pub fn forkpty(amaster: *mut ::c_int,
                name: *mut ::c_char,
                termp: *const termios,
                winp: *const ::winsize) -> ::pid_t;
    pub fn execvpe(file: *const ::c_char, argv: *const *const ::c_char,
                   envp: *const *const ::c_char) -> ::c_int;
}

cfg_if! {
    if #[cfg(any(target_os = "linux",
                 target_os = "emscripten",
                 target_os = "fuchsia"))] {
        mod linux;
        pub use self::linux::*;
    } else if #[cfg(target_os = "android")] {
        mod android;
        pub use self::android::*;
    } else {
        // Unknown target_os
    }
}
