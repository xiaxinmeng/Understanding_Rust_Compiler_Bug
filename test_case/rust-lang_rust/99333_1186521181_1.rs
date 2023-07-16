rust
let mut owners = asset.into_owners_crawler(self.graphql_client).into_iter();
let owner = match owners.next().transpose()? {
    None => unreachable!("owners seems to be empty, should not happened: {asset_id}"),
    Some(owner) => {
        if self.database.owner_exists_or_add(&owner, &asset_id)? {
            return Some(Err(ScrapeError::OwnerRepeat));
        }
        owner
    }
};
