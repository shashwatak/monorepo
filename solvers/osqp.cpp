#include <OsqpEigen/OsqpEigen.h>
#include <Eigen/Dense>
#include <vector>
#include <iostream>

// Example: 2D path with N waypoints, minimize curvature and maximize clearance

int main() {
    constexpr int N = 10; // Number of waypoints
    constexpr double clearance_weight = 1.0;
    constexpr double curvature_weight = 10.0;

    // Example: fixed start and end points
    Eigen::Vector2d start(0, 0), end(10, 0);

    // Clearance: for demo, set a fake clearance vector (e.g., distance to nearest obstacle at each waypoint)
    Eigen::VectorXd clearance(N);
    clearance.setConstant(2.0); // All waypoints have clearance of 2.0 units

    // Decision variables: x = [x0, y0, x1, y1, ..., xN-1, yN-1]
    int num_vars = 2 * N;

    // Objective: Minimize sum of squared curvature + maximize clearance
    // Curvature at i: ||p_{i+1} - 2*p_i + p_{i-1}||^2
    Eigen::SparseMatrix<double> P(num_vars, num_vars);
    Eigen::VectorXd q = Eigen::VectorXd::Zero(num_vars);

    std::vector<Eigen::Triplet<double>> triplets;

    // Curvature term
    for (int i = 1; i < N - 1; ++i) {
        for (int d = 0; d < 2; ++d) {
            int idx_prev = 2 * (i - 1) + d;
            int idx = 2 * i + d;
            int idx_next = 2 * (i + 1) + d;

            triplets.emplace_back(idx_prev, idx_prev, curvature_weight);
            triplets.emplace_back(idx, idx, 4 * curvature_weight);
            triplets.emplace_back(idx_next, idx_next, curvature_weight);

            triplets.emplace_back(idx_prev, idx, -2 * curvature_weight);
            triplets.emplace_back(idx, idx_prev, -2 * curvature_weight);

            triplets.emplace_back(idx_next, idx, -2 * curvature_weight);
            triplets.emplace_back(idx, idx_next, -2 * curvature_weight);

            triplets.emplace_back(idx_prev, idx_next, curvature_weight);
            triplets.emplace_back(idx_next, idx_prev, curvature_weight);
        }
    }

    // Clearance term: -clearance_weight * sum_i clearance[i]
    for (int i = 0; i < N; ++i) {
        for (int d = 0; d < 2; ++d) {
            int idx = 2 * i + d;
            q(idx) -= clearance_weight * clearance(i);
        }
    }

    P.setFromTriplets(triplets.begin(), triplets.end());

    // Constraints: fix start and end points
    Eigen::SparseMatrix<double> A(4, num_vars);
    Eigen::VectorXd l = Eigen::VectorXd::Zero(4);
    Eigen::VectorXd u = Eigen::VectorXd::Zero(4);

    std::vector<Eigen::Triplet<double>> A_triplets;
    // x0 = start.x
    A_triplets.emplace_back(0, 0, 1.0);
    l(0) = u(0) = start.x();
    // y0 = start.y
    A_triplets.emplace_back(1, 1, 1.0);
    l(1) = u(1) = start.y();
    // xN-1 = end.x
    A_triplets.emplace_back(2, 2 * (N - 1), 1.0);
    l(2) = u(2) = end.x();
    // yN-1 = end.y
    A_triplets.emplace_back(3, 2 * (N - 1) + 1, 1.0);
    l(3) = u(3) = end.y();

    A.setFromTriplets(A_triplets.begin(), A_triplets.end());

    // Setup OSQP
    OsqpEigen::Solver solver;
    solver.settings()->setVerbosity(false);
    solver.settings()->setWarmStart(true);

    solver.data()->setNumberOfVariables(num_vars);
    solver.data()->setNumberOfConstraints(4);

    if (!solver.data()->setHessianMatrix(P)) return 1;
    if (!solver.data()->setGradient(q)) return 1;
    if (!solver.data()->setLinearConstraintsMatrix(A)) return 1;
    if (!solver.data()->setLowerBound(l)) return 1;
    if (!solver.data()->setUpperBound(u)) return 1;

    if (!solver.initSolver()) return 1;

    if (solver.solveProblem() != OsqpEigen::ErrorExitFlag::NoError) {
        std::cout << "Solver failed\n";
        return 1;
    }

    Eigen::VectorXd sol = solver.getSolution();
    std::cout << "Optimized path:\n";
    for (int i = 0; i < N; ++i) {
        std::cout << sol(2 * i) << ", " << sol(2 * i + 1) << "\n";
    }
    return 0;
}