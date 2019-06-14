pragma solidity >=0.4.22 <0.7.0;

contract CarReport {
    address servidor;
    address veiculo;
    string report;
    
    function set_report(address a, string memory s) public {
        servidor = msg.sender;
        report = s;
    }

	function get_report() public view returns (string memory) {
			return report;
	}
}
