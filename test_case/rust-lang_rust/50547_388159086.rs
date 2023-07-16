cs
[Fact]
public async Task Should_UpdateTrackableStatus()
{
	var web3 = TestHelper.GetWeb3();
	var factory = await SeasonFactory.DeployAsync(web3);
	var season = await factory.CreateSeasonAsync(DateTimeOffset.UtcNow, DateTimeOffset.UtcNow.AddDays(1));
	var request = await season.GetOrCreateRequestAsync("123");

	var trackableStatus = new StatusUpdate(DateTimeOffset.UtcNow, Request.TrackableStatuses.First(), "Trackable status");
	var nonTrackableStatus = new StatusUpdate(DateTimeOffset.UtcNow, 0, "Nontrackable status");

	await request.UpdateStatusAsync(trackableStatus);
	await request.UpdateStatusAsync(nonTrackableStatus);

	var statuses = await request.GetStatusesAsync();

	Assert.Single(statuses);
	Assert.Equal(trackableStatus, statuses.Single());
}
