/**
 * ZooKnapsack.java
 *
 * This program solves the 0/1 Knapsack problem using dynamic programming.
 * The scenario involves tourists selecting wildlife photo equipment to bring on a trip.
 * Each item has a weight and a biodiversity score. The goal is to choose items that
 * maximize the total biodiversity score without exceeding the luggage weight limit.
 */

 public class ZooKnapsack {

    /**
     * This class represents a piece of equipment with a weight and biodiversity score.
     */
    static class Item {
        int weight;  // The weight of this item in units
        int score;   // The biodiversity score (value) associated with this item

        // Constructor to initialize the item
        Item(int weight, int score) {
            this.weight = weight;
            this.score = score;
        }
    }

    /**
     * Main method: builds the DP table and determines the optimal item selection.
     * @param args command-line arguments
     */
    public static void main(String[] args) {
        // Initialize the list of available items (index matches Item ID in problem)
        Item[] items = {
            new Item(6, 1600),  // Item 1: weight = 6, score = 1600
            new Item(4, 1000),  // Item 2: weight = 4, score = 1000
            new Item(5, 1800),  // Item 3: weight = 5, score = 1800
            new Item(3, 1200),  // Item 4: weight = 3, score = 1200
            new Item(7, 2000)   // Item 5: weight = 7, score = 2000
        };

        int W = 18;  // The maximum total weight allowed in the luggage
        int n = items.length;  // Total number of items

        // To create a 2D DP table: dp[i][w] = max score using first i items with weight limit w
        int[][] dp = new int[n + 1][W + 1];

        // Fills the DP table using bottom-up dynamic programming
        for (int i = 1; i <= n; i++) {
            for (int w = 0; w <= W; w++) {
                // If the item fits into the current weight limit
                if (items[i - 1].weight <= w) {
                    // Decide whether to include or exclude the item based on max score
                    dp[i][w] = Math.max(
                        dp[i - 1][w],  // Exclude the item
                        dp[i - 1][w - items[i - 1].weight] + items[i - 1].score  // Include the item
                    );
                } else {
                    // Item too heavy — carry over the previous value
                    dp[i][w] = dp[i - 1][w];
                }
            }
        }

        // Output the best score possible within the weight limit
        System.out.println("Maximum Biodiversity Score: " + dp[n][W]);

        // Backtrack through the table to find which items were included
        System.out.print("Selected Item IDs: ");
        int w = W;  // Start from the maximum weight
        for (int i = n; i > 0; i--) {
            // If the value in dp[i][w] differs from dp[i-1][w], this item was used
            if (dp[i][w] != dp[i - 1][w]) {
                System.out.print(i + " ");  // Output item ID (1-indexed)
                w -= items[i - 1].weight;  // Reduce the remaining weight
            }
        }
        System.out.println();  // Print newline after item list
    }
}
