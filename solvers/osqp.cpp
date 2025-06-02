#include <OsqpEigen/OsqpEigen.h>
#include <iostream> // For debug logging
#include <optional>

#include <Eigen/Dense>
#include <vector>
#include <iostream>

// Example: 2D path with N waypoints, minimize curvature and maximize clearance
struct Obstacle
{
    Eigen::Vector2d center;
    Eigen::Vector2d half_extent; // half width and half height
    bool contains(const Eigen::Vector2d &p) const
    {
        return (std::abs(p.x() - center.x()) <= half_extent.x()) &&
               (std::abs(p.y() - center.y()) <= half_extent.y());
    }
    double clearance(const Eigen::Vector2d &p) const
    {
        // Distance to the edge of the box (0 if inside)
        Eigen::Vector2d d = (p - center).cwiseAbs() - half_extent;
        return d.cwiseMax(0.0).norm();
    }
};

// Refactored function returning std::optional
std::optional<Eigen::MatrixXd> optimizePath(const Eigen::Vector2d& start, const Eigen::Vector2d& end, const Obstacle& obs, int N = 10)
{
    constexpr double clearance_weight = 1.0;
    constexpr double curvature_weight = 10.0;

    int num_vars = 2 * N;

    Eigen::SparseMatrix<double> P(num_vars, num_vars);
    Eigen::VectorXd q = Eigen::VectorXd::Zero(num_vars);

    std::vector<Eigen::Triplet<double>> triplets;

    // Clearance: distance to obstacle for each waypoint
    Eigen::VectorXd clearance(N);
    for (int i = 0; i < N; ++i)
    {
        double alpha = double(i) / (N - 1);
        Eigen::Vector2d p = (1 - alpha) * start + alpha * end;
        clearance(i) = obs.clearance(p);
    }
    // Curvature term
    for (int i = 1; i < N - 1; ++i)
    {
        for (int d = 0; d < 2; ++d)
        {
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
    for (int i = 0; i < N; ++i)
    {
        for (int d = 0; d < 2; ++d)
        {
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

    if (!solver.data()->setHessianMatrix(P))
        return std::nullopt;
    if (!solver.data()->setGradient(q))
        return std::nullopt;
    if (!solver.data()->setLinearConstraintsMatrix(A))
        return std::nullopt;
    if (!solver.data()->setLowerBound(l))
        return std::nullopt;
    if (!solver.data()->setUpperBound(u))
        return std::nullopt;

    if (!solver.initSolver())
        return std::nullopt;

    if (solver.solveProblem() != OsqpEigen::ErrorExitFlag::NoError)
        return std::nullopt;

    Eigen::VectorXd sol = solver.getSolution();
    Eigen::MatrixXd path(N, 2);
    for (int i = 0; i < N; ++i)
    {
        path(i, 0) = sol(2 * i);
        path(i, 1) = sol(2 * i + 1);
    }
    return path;
}


int main()
{
    constexpr int N = 10; // Number of waypoints
    constexpr double clearance_weight = 1.0;
    constexpr double curvature_weight = 10.0;

    // Example: fixed start and end points
    Eigen::Vector2d start(0, 0), end(10, 0);

    // 2x2 box centered at (5, 0)
    Obstacle obs{Eigen::Vector2d(5, 0), Eigen::Vector2d(1, 1)};

    // Optimize path
    auto result = optimizePath(start, end, obs, N);
    if (!result)
    {
        std::cerr << "Path optimization failed.\n";
        return -1;
    }
    Eigen::MatrixXd sol = *result;
    std::cout << "Optimized path with " << N << " waypoints:\n";
    std::cout << "Start: (" << start.x() << ", " << start.y() << ")\n";
    std::cout << "End: (" << end.x() << ", " << end.y() << ")\n";
    std::cout << "Obstacle center: (" << obs.center.x() << ", " << obs.center.y() << "), "
              << "half extent: (" << obs.half_extent.x() << ", " << obs.half_extent.y() << ")\n";
    std::cout << "Clearance weight: " << clearance_weight << ", Curvature weight: " << curvature_weight << "\n";
    std::cout << "Path waypoints:\n";
    for (int i = 0; i < N; ++i)
    {
        std::cout << "Waypoint " << i << ": (" << sol(i, 0) << ", " << sol(i, 1) << ")\n";
    }
    std::cout << "Clearance at waypoints:\n";
    for (int i = 0; i < N; ++i)
    {
        Eigen::Vector2d p = sol.row(i);
        double clearance = obs.clearance(p);
        std::cout << "Waypoint " << i << ": " << clearance << "\n";
    }
    std::cout << "Optimized path:\n";
    for (int i = 0; i < N; ++i)
    {
        std::cout << sol(2 * i) << ", " << sol(2 * i + 1) << "\n";
    }
    return 0;
}
