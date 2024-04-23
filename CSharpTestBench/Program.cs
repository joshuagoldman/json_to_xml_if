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

  [System.Security.SuppressUnmanagedCodeSecurity]
  [DllImport("json_xml_parser", EntryPoint = "stored_procedure_to_json", CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
  static extern String StoredProcedureToJson(string plSqlPackageStr);

  static void Main(string[] args)
  {

    var assembly = Assembly.GetExecutingAssembly();
    using var resourceJson = assembly.GetManifestResourceStream("CSharpTestBench.EmbeddedResources.json_example.json");
    using var resourceXml = assembly.GetManifestResourceStream("CSharpTestBench.EmbeddedResources.xml_example.xml");
    using var resourcePlSqlPackage = assembly.GetManifestResourceStream("CSharpTestBench.EmbeddedResources.xml_example.xml");
    using var srJson = new StreamReader(resourceJson);
    string jsonContent = srJson.ReadToEnd();
    using var srXml = new StreamReader(resourceXml);
    string xmlContent = srXml.ReadToEnd();
    using var srPlSqlPackage = new StreamReader(resourcePlSqlPackage);
    string plSqlPackageContent = srPlSqlPackage.ReadToEnd();

    try
    {
      var res_json = XmlToJson(xmlContent, true);
      var res_xml = JsonToXml(jsonContent, true, "parameters");
      var res_pl_sql_package = StoredProcedureToJson(plSqlPackageContent);

      string basePath = String.Empty;
      if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
      {
        basePath = "/home/joshua/Public/Tests";
      }
      else
      {
        basePath = "C:/Data";
      }

      File.WriteAllText($"{basePath}/xml_run_fr_csharp.xml", res_xml);
      File.WriteAllText($"{basePath}/json_run_fr_csharp.json", res_json);
      File.WriteAllText($"{basePath}/pl_sql_json_run_fr_csharp.json", res_json);
    }
    catch (System.Exception e)
    {
      Console.Write(e.ToString());
    }
  }
}
