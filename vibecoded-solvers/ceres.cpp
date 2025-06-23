#include <iostream>
#include <vector>
#include <cmath> // For std::abs, std::max

#include "ceres/ceres.h"
#include "glog/logging.h"

// --- Cost Functor for Comfort (Minimizing Jerk) ---
struct JerkComfortCostFunctor {
    JerkComfortCostFunctor(double delta_t, double weight)
        : delta_t_(delta_t), weight_(weight) {}

    template <typename T>
    bool operator()(const T* const p_i,    // position at time i
                    const T* const p_ip1,  // position at time i+1
                    const T* const p_ip2,  // position at time i+2
                    const T* const p_ip3,  // position at time i+3
                    T* residual) const {
        // Jerk: (p_ip3 - 3*p_ip2 + 3*p_ip1 - p_i) / (delta_t^3)
        T jerk = (p_ip3[0] - T(3.0) * p_ip2[0] + T(3.0) * p_ip1[0] - p_i[0]) /
                 (T(delta_t_ * delta_t_ * delta_t_));
        residual[0] = weight_ * jerk;
        return true;
    }

private:
    const double delta_t_;
    const double weight_;
};

// --- Cost Functor for Acceleration Constraints ---
// Penalizes |acceleration| > a_limit
struct AccelConstraintCostFunctor {
    AccelConstraintCostFunctor(double delta_t, double a_limit_abs, double weight)
        : delta_t_(delta_t), a_limit_abs_(a_limit_abs), weight_(weight) {
        CHECK_GT(a_limit_abs, 0.0) << "Absolute acceleration limit must be positive.";
    }

    template <typename T>
    bool operator()(const T* const p_i,    // position at time i
                    const T* const p_ip1,  // position at time i+1
                    const T* const p_ip2,  // position at time i+2
                    T* residuals) const { // Expecting 2 residuals if using separate upper/lower
        // Acceleration: (p_ip2 - 2*p_ip1 + p_i) / (delta_t^2)
        T accel = (p_ip2[0] - T(2.0) * p_ip1[0] + p_i[0]) / T(delta_t_ * delta_t_);

        // Penalty for accel > a_limit_abs_
        residuals[0] = weight_ * ceres::fmax(T(0.0), accel - T(a_limit_abs_));

        // Penalty for accel < -a_limit_abs_  (which is -accel > a_limit_abs_)
        residuals[1] = weight_ * ceres::fmax(T(0.0), -accel - T(a_limit_abs_));
        
        return true;
    }

private:
    const double delta_t_;
    const double a_limit_abs_; // Absolute limit for acceleration (e.g., 5.0 for +/- 5.0)
    const double weight_;      // To control the "hardness" of the constraint
};

// --- Cost Functor for Jerk Constraints ---
// Penalizes |jerk| > j_limit
struct JerkConstraintCostFunctor {
    JerkConstraintCostFunctor(double delta_t, double j_limit_abs, double weight)
        : delta_t_(delta_t), j_limit_abs_(j_limit_abs), weight_(weight) {
        CHECK_GT(j_limit_abs, 0.0) << "Absolute jerk limit must be positive.";
    }

    template <typename T>
    bool operator()(const T* const p_i,    // position at time i
                    const T* const p_ip1,  // position at time i+1
                    const T* const p_ip2,  // position at time i+2
                    const T* const p_ip3,  // position at time i+3
                    T* residuals) const { // Expecting 2 residuals
        // Jerk: (p_ip3 - 3*p_ip2 + 3*p_ip1 - p_i) / (delta_t^3)
        T jerk = (p_ip3[0] - T(3.0) * p_ip2[0] + T(3.0) * p_ip1[0] - p_i[0]) /
                 (T(delta_t_ * delta_t_ * delta_t_));

        // Penalty for jerk > j_limit_abs_
        residuals[0] = weight_ * ceres::fmax(T(0.0), jerk - T(j_limit_abs_));
        
        // Penalty for jerk < -j_limit_abs_ (which is -jerk > j_limit_abs_)
        residuals[1] = weight_ * ceres::fmax(T(0.0), -jerk - T(j_limit_abs_));
        
        return true;
    }

private:
    const double delta_t_;
    const double j_limit_abs_; // Absolute limit for jerk (e.g., 3.0 for +/- 3.0)
    const double weight_;
};


int main(int argc, char** argv) {
    google::InitGoogleLogging(argv[0]);

    // 1. Problem Parameters
    const int num_points = 50;       // Number of discrete points in the trajectory
    const double total_time = 5.0;   // Total duration of the trajectory in seconds
    const double delta_t = total_time / (num_points - 1); // Time step between points

    const double p_start = 0.0;      // Starting position
    const double p_end = 10.0;       // Ending position

    const double accel_limit_abs = 5.0; // m/s^2
    const double jerk_limit_abs = 3.0;  // m/s^3

    // Weights for different cost terms (tune these based on desired behavior)
    const double comfort_weight = 1.0;          // Weight for minimizing jerk (comfort)
    const double accel_constraint_weight = 100.0; // Weight for acceleration constraint penalty
    const double jerk_constraint_weight = 100.0;  // Weight for jerk constraint penalty

    // 2. Optimization Variables: Positions p_i
    // Initialize with a linear interpolation between start and end as a guess
    std::vector<double> positions(num_points);
    for (int i = 0; i < num_points; ++i) {
        positions[i] = p_start + (p_end - p_start) * static_cast<double>(i) / (num_points - 1);
    }

    // 3. Create Ceres Problem
    ceres::Problem problem;

    // Add parameter blocks for each position variable
    for (int i = 0; i < num_points; ++i) {
        problem.AddParameterBlock(&positions[i], 1); // Each position is a 1D variable
    }

    // 4. Add Cost Terms

    // a. Jerk Comfort Cost (minimize sum of squared jerks)
    // Jerk is defined over 4 points (i, i+1, i+2, i+3)
    for (int i = 0; i < num_points - 3; ++i) {
        ceres::CostFunction* comfort_cost =
            new ceres::AutoDiffCostFunction<JerkComfortCostFunctor, 1, 1, 1, 1, 1>(
                new JerkComfortCostFunctor(delta_t, comfort_weight));
        problem.AddResidualBlock(comfort_cost,
                                 nullptr, // No loss function for the primary objective, or use HuberLoss for robustness
                                 &positions[i],
                                 &positions[i+1],
                                 &positions[i+2],
                                 &positions[i+3]);
    }

    // b. Acceleration Constraint Penalty
    // Acceleration is defined over 3 points (i, i+1, i+2)
    for (int i = 0; i < num_points - 2; ++i) {
        ceres::CostFunction* accel_constraint_cost =
            new ceres::AutoDiffCostFunction<AccelConstraintCostFunctor, 2, 1, 1, 1>(
                new AccelConstraintCostFunctor(delta_t, accel_limit_abs, accel_constraint_weight));
        problem.AddResidualBlock(accel_constraint_cost,
                                 nullptr, // No loss function, or a scaled one
                                 &positions[i],
                                 &positions[i+1],
                                 &positions[i+2]);
    }

    // c. Jerk Constraint Penalty
    // Jerk is defined over 4 points (i, i+1, i+2, i+3)
    for (int i = 0; i < num_points - 3; ++i) {
        ceres::CostFunction* jerk_constraint_cost =
            new ceres::AutoDiffCostFunction<JerkConstraintCostFunctor, 2, 1, 1, 1, 1>(
                new JerkConstraintCostFunctor(delta_t, jerk_limit_abs, jerk_constraint_weight));
        problem.AddResidualBlock(jerk_constraint_cost,
                                 nullptr, // No loss function
                                 &positions[i],
                                 &positions[i+1],
                                 &positions[i+2],
                                 &positions[i+3]);
    }

    // 5. Add Constraints on Start and End Points
    // Fix the first and last position
    problem.SetParameterBlockConstant(&positions[0]);
    problem.SetParameterBlockConstant(&positions[num_points - 1]);
    // The values of positions[0] and positions[num_points-1] are already set from initialization.

    // Optional: Set initial velocity/acceleration to zero if desired (more complex)
    // This would involve adding constraints on the first few points, e.g.,
    // p0 = p1 (zero initial velocity if dt is small or if using backward difference for v0)
    // or p0 = p_start, p1 = p_start (zero initial velocity)
    // or p0 = p_start, p1 = p_start, p2 = p_start (zero initial velocity and acceleration)
    // For simplicity, we are not explicitly setting initial/final velocity/accel to zero here,
    // but the jerk minimization will naturally try to make them smooth.

    // 6. Configure and Run the Solver
    ceres::Solver::Options options;
    options.linear_solver_type = ceres::SPARSE_NORMAL_CHOLESKY; // Good for this type of problem
    // options.linear_solver_type = ceres::DENSE_QR; // Alternative for smaller problems
    options.minimizer_progress_to_stdout = true;
    options.max_num_iterations = 200;
    options.function_tolerance = 1e-8;
    options.gradient_tolerance = 1e-8;
    options.parameter_tolerance = 1e-8;


    ceres::Solver::Summary summary;
    ceres::Solve(options, &problem, &summary);

    // 7. Output Results
    std::cout << summary.BriefReport() << "\n";

    if (summary.termination_type == ceres::CONVERGENCE || summary.termination_type == ceres::USER_SUCCESS) {
        std::cout << "\nOptimized Trajectory:\n";
        std::cout << "Time (s), Position (m), Velocity (m/s), Accel (m/s^2), Jerk (m/s^3)\n";
        for (int i = 0; i < num_points; ++i) {
            double t = i * delta_t;
            double p = positions[i];
            double v = NAN, a = NAN, j = NAN;

            if (i < num_points - 1) {
                v = (positions[i+1] - positions[i]) / delta_t;
            }
            if (i < num_points - 2) {
                a = (positions[i+2] - 2.0 * positions[i+1] + positions[i]) / (delta_t * delta_t);
            }
            if (i < num_points - 3) {
                j = (positions[i+3] - 3.0 * positions[i+2] + 3.0 * positions[i+1] - positions[i]) / (delta_t * delta_t * delta_t);
            }
            printf("%.3f, %.4f, %.4f, %.4f, %.4f\n", t, p, v, a, j);
        }
    } else {
        std::cout << "Solver did not converge.\n";
        std::cout << "Message: " << summary.message << "\n";
    }

    return 0;
}

