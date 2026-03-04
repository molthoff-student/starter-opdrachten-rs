use clap::Parser;
use std::{
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(name = "webserver")]
#[command(about = "Warp static file server")]
pub struct LaunchArgs {
    /// Full socket address (e.g. 127.0.0.1:7878)
    #[arg(long, conflicts_with_all = ["host", "port"])]
    addr: Option<SocketAddr>,

    /// Host to bind to (e.g. 127.0.0.1)
    #[arg(long, requires = "port")]
    host: Option<IpAddr>,

    /// Port to bind to (e.g. 7878)
    #[arg(long, requires = "host")]
    port: Option<u16>,

    /// Path to website directory (e.g. "C://my_website_name//website")
    #[arg(long)]
    path: String,
}

impl LaunchArgs {
    /// Obtains the Socket Address from the launch arguments.
    /// Will panic if the arguments are incorrect.
    pub fn socket_addr(&self) -> SocketAddr {
        if let Some(addr) = self.addr {
            addr
        } else {
            let host = self.host.expect("host required");
            let port = self.port.expect("port required");
            SocketAddr::new(host, port)
        }
    }

    /// Obtains the website's files to load.
    /// Will panic if it isn't a path.
    pub fn website_dir(&self) -> PathBuf {
        let path = Path::new(&self.path);

        if path.is_dir() {
            path.canonicalize()
                .expect("Failed to canonicalize website directory")
        } else {
            panic!("Path '{}' isn't a valid website directory.", path.display());
        }
    }
}
