#[macro_export]
macro_rules! tx {
    ( $x:expr, $y:expr) => {
        if $x.send($y).is_err() {
            error!("fail to send data!");
            bail!("fail to send data!")
        }
    };
    ($x:expr, $y:expr, $msg:expr) => {
        if $x.send($y).is_err() {
            error!($msg);
            bail!($msg)
        }
    };
}
#[macro_export]
macro_rules! tx_async {
    ( $x:expr, $y:expr) => {
        if $x.send($y).await.is_err() {
            error!("fail to send data!");
            bail!("fail to send data!")
        }
    };
    ($x:expr, $y:expr, $msg:expr) => {
        if $x.send($y).await.is_err() {
            error!($msg);
            bail!($msg)
        }
    };
}
#[macro_export]
macro_rules! rx {
    ( $x:expr) => {
        match $x.recv() {
            Ok(val) => val,
            Err(e) => {
                error!("{:?}", e);
                Err(e)?
            }
        }
    };
    ( $x:expr, $msg:expr) => {
        match $x.recv() {
            Ok(val) => val,
            Err(_) => {
                error!($msg);
                bail!($msg);
            }
        }
    };
}

#[macro_export]
macro_rules! rx_async {
    ( $x:expr) => {
        match $x.recv().await {
            Some(val) => val,
            None => {
                error!("receive none");
                bail!("receive none");
            }
        }
    };
    ( $x:expr, $msg:expr) => {
        match $x.recv().await {
            Some(val) => val,
            None => {
                error!($msg);
                bail!($msg);
            }
        }
    };
}
