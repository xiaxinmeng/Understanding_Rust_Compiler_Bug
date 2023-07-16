cs
public async Task<StatusUpdate[]> GetStatusesAsync()
{
	int statusUpdatesCount = await Contract.GetFunction("getStatusUpdatesCount").CallAsync<int>();
	var getStatusUpdate = Contract.GetFunction("getStatusUpdate");
	var tasks = Enumerable.Range(0, statusUpdatesCount).Select(async i =>
	{
		var statusUpdate = await getStatusUpdate.CallDeserializingToObjectAsync<StatusUpdateStruct>(i);
		return new StatusUpdate(XDateTime.UtcOffsetFromTicks(statusUpdate.UpdateDate), statusUpdate.StatusCode, statusUpdate.Note);
	});

	return await Task.WhenAll(tasks);
}
