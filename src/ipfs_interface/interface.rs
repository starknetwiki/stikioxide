use crate::formatters::format_stiki_markdown;
use crate::StikiPage;
use ipfs::{make_ipld, Ipfs, IpfsOptions, IpfsPath, UninitializedIpfs};
use std::convert::TryFrom;
use tokio::task;

#[derive(Debug)]
pub struct IpfsInterface {
    pub ipfs: Ipfs<ipfs::Types>,
}

impl IpfsInterface {
    pub async fn new() -> IpfsInterface {
        let (ipfs, fut): (Ipfs<ipfs::Types>, _) =
            UninitializedIpfs::new(IpfsOptions::inmemory_with_generated_keys())
                .start()
                .await
                .unwrap();
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

    pub(crate) async fn get_stiki(&self, cid: String) -> String {
        let path = IpfsPath::try_from(&*cid).unwrap();
        let path1 = path.sub_path("0").unwrap();
        let stiki_content: StikiPage =
            serde_json::from_str(&format!("{:#?}", self.ipfs.get_dag(path1).await.unwrap()))
                .unwrap();
        format_stiki_markdown(stiki_content.body.clone(), stiki_content.refs)
    }

    // pub(crate) async fn list_files(&self, peer_id: Option<String>) {
    //     let data = self.ipfs.list_pins(None).await;
    // }
}
