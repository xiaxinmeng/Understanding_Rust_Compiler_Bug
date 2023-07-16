C#
var peopleOldEnough = people.Where(p => p.Age >= 18).Select(p => p.FirstName);
