FOR /F  "tokens=4 skip=4" %%g IN ('near dev-deploy') do (SET contract=%%g)
echo %contract% 
set ctrct = %contract%