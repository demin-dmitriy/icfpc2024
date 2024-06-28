# COPY PASTE FROM
#https://developers.google.com/optimization/routing/tsp


"""Simple Travelling Salesperson Problem (TSP) between cities."""

from ortools.constraint_solver import routing_enums_pb2
from ortools.constraint_solver import pywrapcp


def solve_tsp(distance_matrix, start_index: int = 0, search_parameters = None) -> tuple[list[int], float]:
    assert isinstance(distance_matrix[0][0], int)

    # https://developers.google.com/optimization/routing/tsp
    index_manager = pywrapcp.RoutingIndexManager(len(distance_matrix), 1, start_index)
    routing = pywrapcp.RoutingModel(index_manager)

    def distance_callback(from_index, to_index):
        from_node = index_manager.IndexToNode(from_index)
        to_node = index_manager.IndexToNode(to_index)

        return distance_matrix[from_node][to_node]

    transit_callback_index = routing.RegisterTransitCallback(distance_callback)
    routing.SetArcCostEvaluatorOfAllVehicles(transit_callback_index)
    if search_parameters is None:
        search_parameters = pywrapcp.DefaultRoutingSearchParameters()
        search_parameters.first_solution_strategy = (
            routing_enums_pb2.FirstSolutionStrategy.PATH_CHEAPEST_ARC
        )

    solution = routing.SolveWithParameters(search_parameters)
    assert solution is not None

    route = []
    cost = 0

    index = routing.Start(0)
    while not routing.IsEnd(index):
        route.append(index_manager.IndexToNode(index))
        previous_index = index
        index = solution.Value(routing.NextVar(index))
        cost += routing.GetArcCostForVehicle(previous_index, index, 0)

    return route, cost
