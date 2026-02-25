use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttentionWeights {
    #[serde(flatten)]
    pub layers: HashMap<String, Vec<f32>>,
}

pub struct AttentionModel {
    pub vertex_key_size: usize,
    pub vertex_value_size: usize,
    pub muscle_key_size: usize,
    pub num_heads: usize,
    pub weights: HashMap<String, Vec<f32>>,
}

impl AttentionModel {
    pub fn new(args_json: &str, weights_json: &str) -> Self {
        let args: serde_json::Value = serde_json::from_str(args_json).unwrap();
        let weights: HashMap<String, Vec<f32>> = serde_json::from_str(weights_json).unwrap();

        Self {
            vertex_key_size: args["vertex_key_size"].as_u64().unwrap() as usize,
            vertex_value_size: args["vertex_value_size"].as_u64().unwrap() as usize,
            muscle_key_size: args["muscle_key_size"].as_u64().unwrap() as usize,
            num_heads: args["num_heads"].as_u64().unwrap() as usize,
            weights,
        }
    }

    fn linear(&self, input: &[f32], weight_name: &str, bias_name: &str, in_features: usize, out_features: usize) -> Vec<f32> {
        let weight = &self.weights[weight_name];
        let bias = &self.weights[bias_name];
        let mut output = vec![0.0; out_features];

        for i in 0..out_features {
            let mut sum = bias[i];
            for j in 0..in_features {
                sum += input[j] * weight[i * in_features + j];
            }
            output[i] = sum;
        }
        output
    }

    fn relu(input: &mut [f32]) {
        for val in input.iter_mut() {
            if *val < 0.0 {
                *val = 0.0;
            }
        }
    }

    fn tanh(input: &mut [f32]) {
        for val in input.iter_mut() {
            *val = val.tanh();
        }
    }

    pub fn forward(&self, vertex_k: &[f32], muscle_k: &[f32], vertex_v: &[f32], num_vertices: usize, num_muscles: usize) -> Vec<f32> {
        let mut muscle_activations = Vec::with_capacity(num_muscles);

        for m in 0..num_muscles {
            // 1. muscle_k_to_vertex_q
            let m_k = &muscle_k[m * self.muscle_key_size..(m + 1) * self.muscle_key_size];
            
            let mut x = self.linear(m_k, "muscle_k_to_vertex_q.0.weight", "muscle_k_to_vertex_q.0.bias", self.muscle_key_size, 32);
            Self::relu(&mut x);
            let q_all_heads = self.linear(&x, "muscle_k_to_vertex_q.2.weight", "muscle_k_to_vertex_q.2.bias", 32, self.num_heads * self.vertex_key_size);

            // 2. Attention
            let mut head_outputs = vec![0.0; self.num_heads * self.vertex_value_size];
            
            for h in 0..self.num_heads {
                let q_head = &q_all_heads[h * self.vertex_key_size..(h + 1) * self.vertex_key_size];
                
                let mut attention_scores = vec![0.0; num_vertices];
                let mut max_score = f32::NEG_INFINITY;

                for v in 0..num_vertices {
                    let k_v = &vertex_k[v * self.vertex_key_size..(v + 1) * self.vertex_key_size];
                    let mut score = 0.0;
                    for i in 0..self.vertex_key_size {
                        score += q_head[i] * k_v[i];
                    }
                    attention_scores[v] = score;
                    if score > max_score {
                        max_score = score;
                    }
                }

                // Softmax
                let mut sum_exp = 0.0;
                for score in attention_scores.iter_mut() {
                    *score = (*score - max_score).exp();
                    sum_exp += *score;
                }
                for score in attention_scores.iter_mut() {
                    *score /= sum_exp;
                }

                // Weighted sum of vertex values
                for v in 0..num_vertices {
                    let v_v = &vertex_v[v * self.vertex_value_size..(v + 1) * self.vertex_value_size];
                    for i in 0..self.vertex_value_size {
                        head_outputs[h * self.vertex_value_size + i] += attention_scores[v] * v_v[i];
                    }
                }
            }

            // 3. wv_to_output
            let mut y = self.linear(&head_outputs, "wv_to_output.0.weight", "wv_to_output.0.bias", self.num_heads * self.vertex_value_size, 32);
            Self::relu(&mut y);
            let mut output = self.linear(&y, "wv_to_output.2.weight", "wv_to_output.2.bias", 32, 1);
            Self::tanh(&mut output);
            
            muscle_activations.push(output[0]);
        }

        muscle_activations
    }
}
