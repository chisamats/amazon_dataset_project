Amazon Product Co-purchasing Network Project Writeup

The dataset I worked on was an Amazon product co-purchasing network from March 12 2003. The description says, “Network was collected by crawling the Amazon website. It is based on Customers Who Bought This Item Also Bought feature of the Amazon website. If a product i is frequently co-purchased with product j, the graph contains a directed edge from i to j.” 
I chose this dataset because I am a frequent user of Amazon and I am curious to see how effective the certain features of the website are in selling more products to users. I personally look at the “Customers Who Bought This Item Also Bought” feature but I do not often actually buy anything from that category. There is also a lot of data to work with because Amazon is a big site with a large amount of users. 


Power Law Fit
I wanted to see whether this network followed a power law fit as many networks do. This would mean that some products are very well connected with others, or that those products are commonly bought with other products using the Customers Who Bought This Item Also Bought feature, while most products are not as connected and probably only bought with one other product commonly. 

read_file(): This function reads in the file and puts the data into a graph with to-node and from-node just as in the file. 
in_degree(): This function takes the graph made from read_file() and makes it into an in degree graph, with a node and the number of edges coming into it. 
degree_distribution(): This function takes the in degree graph made from read_file() into a degree distribution graph by taking the degree of incoming edges to a node and the frequency of nodes with that amount of incoming edges. 
top_five(): This function takes the in degree graph and puts it into vector form to be able to sort the nodes by number of incoming edges. It takes the top five nodeswith the most number of incoming edges and put that information into a Vec<usize, usize> of product node number and number of other products it was co-purchased with which gets returned. 
bottom(): This function also takes the in degree graph and puts it into vector form. However, we sort this from least to greatest number of incoming edges. Because there are very many nodes that only have 1 product it has been co-purchased with, instead of taking the bottom five, this function finds out how many products have only been co-purchased with 1 other product using the for loop and an if statement, storing the number of this type of product in a variable called count. Count gets returned at the end. 
build_chart(): This part builds a degree distribution plot on a log log scale to be able to see the power law fit easier as it is more linear. We use the minimum and maximum values of the degree distribution HashMap’s degrees and frequencies to set the lengths of the axes. The data points get put into (degree, frequency) format and plotted as black dots on the graph. 

Graph
The graph follows the power law fit that we had predicted for it with many products having very few co-purchased products (upper left side of graph) and a few products with many co-purchased products (bottom right corner). 
Output
Most Co-purchased Products(product number, connections): [(32, 2747), (335, 2247), (8, 2247), (2887, 1413), (12588, 1282)]
This data shows us that product numbers 32, 335, 8, 2887, and 12588 were co-purchased with the most other products. The highest number of co-purchased products was 2747 products for product number 32. 

What this could tell us:
Information like this could potentially help Amazon to understand which products to push more on their website as they are more likely to get co-purchased through the Customers Who Bought this Item Also Bought feature. 

Number of Products only Co-purchased with 1 other item: 62552
This tells us that 62,552 products were bought with only 1 other item through the feature. 

What this could tell us:
This information could help Amazon decide whether it is worth having this feature as so many items only get co-purchased with 1 other item. Maybe other features that will have customers adding more to their cart or a different algorithm for this feature would help get more items being purchased together. 


Six Degrees of Separation
Separate from the degree distribution, I also wanted to see if this dataset also fell within the six degrees of separation rule. Because this dataset is so large, for the six degrees of separation, I used 40,072 of the nodes which is 1/10th of all nodes.

out_degree_adj_list(): This function takes the from-node/to-node graph that was made initially from reading the file in main and turns it into an out degree adjacency list with 1/10th of all nodes. 
bfs_degrees_of_separation(): This function runs bfs on a graph from a starting node to all its neighbors. 
graph_degrees_of_separation(): This function uses bfs_degrees_of_separation() and uses a for loop to run on all nodes. It keeps track of how many paths are larger than 6 and prints out the number and ratio to all paths. 

Output
1. 470/1508312569 = 0.0000 paths between nodes are greater than 6
2. 753/1508312569 = 0.0000 paths between nodes are greater than 6
3. 462/1508312569 = 0.0000 paths between nodes are greater than 6
4. 661/1508312569 = 0.0000 paths between nodes are greater than 6
5. 492/1508312569 = 0.0000 paths between nodes are greater than 6
After running this 5 different times, the data shows that pretty much 0% of paths are greater than 6.

What this could tell us:
This means six degrees of separation does not do well to describe this graph. This may mean that a lot of these products are not of the same category of product and that they do not often go together as co-purchasing items. Amazon may want to suggest more similar products if they would like this feature to bring them more sales, or otherwise create a new feature. 
