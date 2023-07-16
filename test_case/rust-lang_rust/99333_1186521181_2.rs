rust
let mut owners = asset.into_owners_crawler(self.graphql_client).into_iter();
let owner = match owners.next().transpose()? {
    None => unreachable!("owners seems to be empty, should not happened: {asset_id}"),
    Some(owner) => {
        self.database.add_owner(&owner, &asset_id)?;
        owner
    }
};
