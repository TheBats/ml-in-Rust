using DataFrames
using CSV
using Plots
using Lathe
using GLM
using Statistics

df = DataFrame(CSV.File("src/housing.csv"))

train =  first(df[:, [:median_house_value,:housing_median_age]], 16513)
test = last(df[:, [:housing_median_age]], 4127)

fm = @formula(median_house_value ~ housing_median_age)
linearRegressor = lm(fm, train)

# println(linearRegressor)

predictions = predict(linearRegressor, test)

println(first(test, 1))
println(first(predictions, 1))