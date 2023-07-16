
T1. std::call_once(initializeSpillPlacementPassFlag)
T1.   acquires global once_mutex
T1.   acquires mutex for initializeSpillPlacementPassFlag
T1.   unlocks global once_mutex 
T1.   proceeds with initializeSpillPlacementPassOnce

T5. std::call_once(initializeSpillPlacementPassFlag)
T5.   acquires global once_mutex
T5.   waits for initializeSpillPlacementPassFlag (hold by T1)

T1.   SpillPlacement initializes a dependency - EdgeBundles pass
T1.   std::call_once(initializeEdgeBundlesPassFlag)
T1.     waits for global once_mutex (hold by T5)
