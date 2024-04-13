// See https://aka.ms/new-console-template for more information
using System.Reflection;
using System.Runtime.InteropServices;

class Program
{
  [System.Security.SuppressUnmanagedCodeSecurity]
  [DllImport("json_xml_parser", EntryPoint = "json_to_xml", CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
  static extern String JsonToXml(string json_str, bool to_snake_case, string root_name);

  [System.Security.SuppressUnmanagedCodeSecurity]
  [DllImport("json_xml_parser", EntryPoint = "xml_to_json", CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
  static extern String XmlToJson(string xml_str, bool to_camel_case);

  static void Main(string[] args)
  {

    var assembly = Assembly.GetExecutingAssembly();
    using var resourceJson = assembly.GetManifestResourceStream("CSharpTestBench.EmbeddedResources.json_example.json");
    using var resourceXml = assembly.GetManifestResourceStream("CSharpTestBench.EmbeddedResources.xml_example.xml");
    using var srJson = new StreamReader(resourceJson);
    string jsonContent = srJson.ReadToEnd();
    using var srXml = new StreamReader(resourceXml);
    string xmlContent = srXml.ReadToEnd();

    try
    {
      var res_json = XmlToJson(xmlContent, true);
      var res_xml = JsonToXml(jsonContent, true, "parameters");


#if Windows
      File.WriteAllText("C:/Data/xml_run_fr_csharp.xml", res_xml);
      File.WriteAllText("C:/Data/joshua/Public/Tests/json_run_fr_csharp.json", res_json);
#else 
      File.WriteAllText("/home/joshua/Public/Tests/xml_run_fr_csharp.xml", res_xml);
      File.WriteAllText("/home/joshua/Public/Tests/json_run_fr_csharp.json", res_json);

#endif

    }
    catch (System.Exception e)
    {
      Console.Write(e.ToString());
    }
  }
}
