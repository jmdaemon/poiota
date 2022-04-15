import pdb
from pyrosm import OSM
# from pyrosm import get_data

# Pyrosm comes with a couple of test datasets 
# that can be used straight away without
# downloading anything
# fp = get_data("british-columbia")
fp = './british-columbia-latest.osm.pbf'

# Initialize the OSM parser object
osm = OSM(fp)

def get_drivable_roads(osm):
    ''' Read all drivable roads '''
    # drive_net = osm.get_network(network_type="driving")
    # drive_net = osm.get_network(network_type="cycling")
    drive_net = osm.get_network(network_type="driving+service")
    drive_net.plot()
pdb.run('get_drivable_roads(osm)')
