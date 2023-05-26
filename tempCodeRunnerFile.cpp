#include <iostream>
#include <pybind11/embed.h>

namespace py = pybind11;

int main() {
    py::scoped_interpreter guard{};  // Initialize Python interpreter

    // Import necessary Python modules
    py::module sklearn = py::module::import("sklearn");
    py::module datasets = sklearn.attr("datasets");
    py::module model_selection = sklearn.attr("model_selection");
    py::module tree = sklearn.attr("tree");
    py::module metrics = sklearn.attr("metrics");

    // Load the Iris dataset
    py::object iris = datasets.attr("load_iris")();
    py::object X = iris.attr("data");
    py::object y = iris.attr("target");

    // Split the dataset into training and testing sets
    py::object train_test_split = model_selection.attr("train_test_split");
    py::object args = py::make_tuple(X, y, "test_size"_a = 0.3, "random_state"_a = 1);
    py::tuple train_test_split_result = train_test_split(*args);
    py::object X_train = train_test_split_result[0];
    py::object X_test = train_test_split_result[1];
    py::object y_train = train_test_split_result[2];
    py::object y_test = train_test_split_result[3];

    // Create a decision tree classifier
    py::object DecisionTreeClassifier = tree.attr("DecisionTreeClassifier");
    py::object clf = DecisionTreeClassifier();

    // Train the classifier on the training data
    clf.attr("fit")(X_train, y_train);

    // Make predictions on the testing data
    py::object y_pred = clf.attr("predict")(X_test);

    // Evaluate the model's accuracy
    py::object accuracy = metrics.attr("accuracy_score")(y_test, y_pred);
    std::cout << "Accuracy: " << py::float_(accuracy) << std::endl;

    return 0;
}