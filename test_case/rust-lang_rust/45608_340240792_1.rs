javascript
if (results['others'].length < maxResults && ((query.search && obj.name.indexOf(query.search) != -1) || added === false))
{
    results['others'].push(obj);
}
