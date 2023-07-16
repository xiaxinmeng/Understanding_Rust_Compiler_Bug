rust
let mut owners = asset.into_owners_crawler(self.graphql_client).into_iter();
let owner = match owners.next() {
    Some(Ok(owner)) => match self.database.owner_exists_or_add(&owner, &asset_id) {
        Ok(false) => owner,
        Ok(true) => return Some(Err(ScrapeError::OwnerRepeat)),
        Err(err) => return Some(Err(err.into())),
    },
    Some(Err(error)) => return Some(Err(error)),
    None => unreachable!("owners seems to be empty, should not happened: {asset_id}"),
};
