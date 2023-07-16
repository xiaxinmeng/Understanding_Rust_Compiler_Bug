plain
2019-10-02T23:50:06.0840546Z Version      : 1.0.0
2019-10-02T23:50:06.0840644Z Author       : Microsoft
2019-10-02T23:50:06.0840711Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2019-10-02T23:50:06.0840826Z ==============================================================================
2019-10-02T23:50:06.5013192Z System.NullReferenceException: Object reference not set to an instance of an object.
2019-10-02T23:50:06.5014792Z    at Newtonsoft.Json.JsonReader.get_Path()
2019-10-02T23:50:06.5016014Z    at Newtonsoft.Json.Serialization.JsonSerializerInternalReader.Deserialize(JsonReader reader, Type objectType, Boolean checkAdditionalContent)
2019-10-02T23:50:06.5017101Z    at Newtonsoft.Json.JsonSerializer.DeserializeInternal(JsonReader reader, Type objectType)
2019-10-02T23:50:06.5017610Z    at Newtonsoft.Json.JsonConvert.DeserializeObject(String value, Type type, JsonSerializerSettings settings)
2019-10-02T23:50:06.5017893Z    at Newtonsoft.Json.JsonConvert.DeserializeObject[T](String value, JsonSerializerSettings settings)
2019-10-02T23:50:06.5018153Z    at Agent.PluginHost.Program.Main(String[] args)
2019-10-02T23:50:06.5300003Z ##[error]Exit code 1 returned from process: file name '/home/vsts/agents/2.158.0/bin/Agent.PluginHost', arguments 'task "Agent.Plugins.Repository.CheckoutTask, Agent.Plugins"'.
2019-10-02T23:50:06.5748686Z ##[section]Starting: Upload CPU usage statistics
2019-10-02T23:50:06.5751779Z ==============================================================================
2019-10-02T23:50:06.5751859Z Task         : Bash
2019-10-02T23:50:06.5751929Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-02T23:50:06.7172498Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$CI_JOB_NAME.csv
2019-10-02T23:50:06.7194249Z ========================== Starting Command Output ===========================
2019-10-02T23:50:06.7217431Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/39a33c5e-bc43-4692-b5dc-d87366e7d299.sh
2019-10-02T23:50:10.4129418Z 
2019-10-02T23:50:10.4131251Z The user-provided path cpu-usage.csv does not exist.
2019-10-02T23:50:10.4568255Z ##[error]Bash exited with code '255'.
2019-10-02T23:50:10.4581390Z ##[section]Starting: Checkout
2019-10-02T23:50:10.4583104Z ==============================================================================
2019-10-02T23:50:10.4583237Z Task         : Get sources
2019-10-02T23:50:10.4583304Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
