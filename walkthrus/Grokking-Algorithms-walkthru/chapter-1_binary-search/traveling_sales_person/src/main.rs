struct Plot {
    x: f64,
    y: f64,
}

struct Map {
    plots: Vec<Plot>,
}

impl Map {
    fn optimal_plot_path(&self) -> Vec<usize> {
        if self.plots.is_empty() {
            return Vec::new();
        }
        
        // Track which indices we've already visited
        let mut visited = vec![false; self.plots.len()];
        
        // Start with plot 0 (first plot)
        let mut path = vec![0];
        visited[0] = true;
        
        // Current position is the first plot
        let mut current_idx = 0;
        
        // While we haven't visited all plots
        while path.len() < self.plots.len() {
            let current_plot = &self.plots[current_idx];
            let mut closest_idx = 0;
            let mut min_distance = f64::MAX;
            
            // Find the closest unvisited plot
            for (i, plot) in self.plots.iter().enumerate() {
                // Skip if already visited
                if visited[i] {
                    continue;
                }
                
                // Calculate distance from current plot to this plot
                let dx = plot.x - current_plot.x;
                let dy = plot.y - current_plot.y;
                let distance = (dx.powi(2) + dy.powi(2)).sqrt();
                
                if distance < min_distance {
                    min_distance = distance;
                    closest_idx = i;
                }
            }
            
            // Add the closest plot to our path
            path.push(closest_idx);
            visited[closest_idx] = true;
            
            // Move to the closest plot
            current_idx = closest_idx;
        }
        
        path
    }
}

fn main() {
    // Create plots and map
    let map = Map {
        plots: vec![
            Plot { x: 2.5, y: 3.0 },
            Plot { x: 4.0, y: 1.5 },
            Plot { x: 5.5, y: 7.0 },
            Plot { x: 8.0, y: 2.5 },
        ]
    };
    
    // Get optimal path starting from plot 0
    let path = map.optimal_plot_path();
    
    // Print the path
    println!("Optimal path: {:?}", path);
}