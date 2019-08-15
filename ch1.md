# 1

# 2

# 3

# 4
Most webpages don't have links to themselves, so identity morphisms must also be added. However, many webpages change when reloaded, including their links (morphisms), so the category will not be constant in time, network partitions, and the like. 

A nieve / statically mirrored world wide web can be a category where links are morphisms:
* up to the problems posed by distributed systems (CAP theory and the like)
* provided that compositions of morphisms (i.e. following 2 links sequentially) are procedurally added to the category
* and identity morphisms (reloading) are added

# 5
People are not strictly speaking 'friends' of themselves in facebook, so the same identity issue for the world wide web is present for Facebook. additionaly, the same time variant nature of the world wide web is present here.

I would argue that 'strict' friendship is not composeable, as a friend of my friend is not neccesarily my friend. A'week' definiton of friendship counting 'friend of friend' is composeable. therefore, facebook is a category with people as objects and friendship as morphisms if friend of friend composition is permitted.

# 6
Any directed graph can construct a category provided:
1) identity morphisms (edges) are added to all nodes
2) all compositions of edges are added to the category as new composition of morphism edges.

![Addition of identity morphisms and compositions to a DG to create a category](chapter_1/ch1%236.png)