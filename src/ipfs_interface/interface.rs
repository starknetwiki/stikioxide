use ipfs::{make_ipld, Ipfs, IpfsOptions, IpfsPath, UninitializedIpfs};
// use cid::Cid;
use tokio::task;

pub struct IpfsInterface {
    pub ipfs: Ipfs<ipfs::Types>,
}

impl IpfsInterface {
    pub fn new() -> IpfsInterface {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create event loop");

        let (ipfs, fut): (Ipfs<ipfs::Types>, _) = rt.block_on(async move {
            UninitializedIpfs::new(IpfsOptions::inmemory_with_generated_keys())
                .start()
                .await
                .unwrap()
        });
        task::spawn(fut);
        IpfsInterface { ipfs }
    }

    pub(crate) async fn add_file(&self, contents: String) -> String {
        let cid = self.ipfs.put_dag(make_ipld!(contents)).await.unwrap();
        let root = make_ipld!([cid.clone()]);

        self.ipfs.put_dag(root).await.unwrap();
        self.ipfs.insert_pin(&cid, true).await.unwrap();
        IpfsPath::from(cid).to_string()
    }

    pub(crate) async fn get_file(&self, hash: String) {}
}
