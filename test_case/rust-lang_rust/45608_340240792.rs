javascript
if (results['others'].length < maxResults && ((query.search && obj.name.indexOf(query.search)) || added === false))
{
    results['others'].push(obj);
}
