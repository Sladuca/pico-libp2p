use futures::channel::{mpsc, oneshot};

pub enum ReqRes<Req, Res> {
    WaitSend(mpsc::Sender<(Req, oneshot::Sender<Res>)>),
    WaitRecv(oneshot::Receiver<Res>),
}

pub fn ma_to_tcp_ip_sock_addr(addr: Multiaddr) -> Option<impl ToSocketAddrs> {
    match addr.pop() {
        Some(Protocol::Tcp(port)) => match addr.pop() {
            Some(Protocol::Ip4(addr)) | Some(Protocol::Ip6(addr)) => (addr, port),
            _ => None,
        },
        _ => None,
    }
}
